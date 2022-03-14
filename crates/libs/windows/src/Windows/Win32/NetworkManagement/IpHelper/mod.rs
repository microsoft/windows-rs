#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ADDRESS_FAMILY(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const AF_INET: ADDRESS_FAMILY = ADDRESS_FAMILY(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const AF_INET6: ADDRESS_FAMILY = ADDRESS_FAMILY(23u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const AF_UNSPEC: ADDRESS_FAMILY = ADDRESS_FAMILY(0u32);
impl ::core::marker::Copy for ADDRESS_FAMILY {}
impl ::core::clone::Clone for ADDRESS_FAMILY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADDRESS_FAMILY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ADDRESS_FAMILY {
    type Abi = Self;
}
impl ::core::fmt::Debug for ADDRESS_FAMILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADDRESS_FAMILY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ANY_SIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn AddIPAddress(address: u32, ipmask: u32, ifindex: u32, ntecontext: *mut u32, nteinstance: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddIPAddress(address: u32, ipmask: u32, ifindex: u32, ntecontext: *mut u32, nteinstance: *mut u32) -> u32;
        }
        ::core::mem::transmute(AddIPAddress(::core::mem::transmute(address), ::core::mem::transmute(ipmask), ::core::mem::transmute(ifindex), ::core::mem::transmute(ntecontext), ::core::mem::transmute(nteinstance)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const BEST_IF: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const BEST_ROUTE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const BROADCAST_NODETYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CancelIPChangeNotify(notifyoverlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelIPChangeNotify(notifyoverlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CancelIPChangeNotify(::core::mem::transmute(notifyoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelMibChangeNotify2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(notificationhandle: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelMibChangeNotify2(notificationhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
        }
        CancelMibChangeNotify2(notificationhandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid: *const NET_LUID_LH, crosstimestamp: *mut INTERFACE_HARDWARE_CROSSTIMESTAMP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid: *const NET_LUID_LH, crosstimestamp: *mut INTERFACE_HARDWARE_CROSSTIMESTAMP) -> u32;
        }
        ::core::mem::transmute(CaptureInterfaceHardwareCrossTimestamp(::core::mem::transmute(interfaceluid), ::core::mem::transmute(crosstimestamp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertCompartmentGuidToId(compartmentguid: *const ::windows::core::GUID, compartmentid: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertCompartmentGuidToId(compartmentguid: *const ::windows::core::GUID, compartmentid: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        ConvertCompartmentGuidToId(::core::mem::transmute(compartmentguid), ::core::mem::transmute(compartmentid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertCompartmentIdToGuid(compartmentid: u32, compartmentguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertCompartmentIdToGuid(compartmentid: u32, compartmentguid: *mut ::windows::core::GUID) -> super::super::Foundation::NTSTATUS;
        }
        ConvertCompartmentIdToGuid(::core::mem::transmute(compartmentid), ::core::mem::transmute(compartmentguid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertInterfaceAliasToLuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(interfacealias: Param0, interfaceluid: *mut NET_LUID_LH) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertInterfaceAliasToLuid(interfacealias: ::windows::core::PCWSTR, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
        }
        ConvertInterfaceAliasToLuid(interfacealias.into_param().abi(), ::core::mem::transmute(interfaceluid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertInterfaceGuidToLuid(interfaceguid: *const ::windows::core::GUID, interfaceluid: *mut NET_LUID_LH) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertInterfaceGuidToLuid(interfaceguid: *const ::windows::core::GUID, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
        }
        ConvertInterfaceGuidToLuid(::core::mem::transmute(interfaceguid), ::core::mem::transmute(interfaceluid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertInterfaceIndexToLuid(interfaceindex: u32, interfaceluid: *mut NET_LUID_LH) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertInterfaceIndexToLuid(interfaceindex: u32, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
        }
        ConvertInterfaceIndexToLuid(::core::mem::transmute(interfaceindex), ::core::mem::transmute(interfaceluid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertInterfaceLuidToAlias(interfaceluid: *const NET_LUID_LH, interfacealias: &mut [u16]) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertInterfaceLuidToAlias(interfaceluid: *const NET_LUID_LH, interfacealias: ::windows::core::PWSTR, length: usize) -> super::super::Foundation::NTSTATUS;
        }
        ConvertInterfaceLuidToAlias(::core::mem::transmute(interfaceluid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(interfacealias)), interfacealias.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertInterfaceLuidToGuid(interfaceluid: *const NET_LUID_LH, interfaceguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertInterfaceLuidToGuid(interfaceluid: *const NET_LUID_LH, interfaceguid: *mut ::windows::core::GUID) -> super::super::Foundation::NTSTATUS;
        }
        ConvertInterfaceLuidToGuid(::core::mem::transmute(interfaceluid), ::core::mem::transmute(interfaceguid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertInterfaceLuidToIndex(interfaceluid: *const NET_LUID_LH, interfaceindex: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertInterfaceLuidToIndex(interfaceluid: *const NET_LUID_LH, interfaceindex: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        ConvertInterfaceLuidToIndex(::core::mem::transmute(interfaceluid), ::core::mem::transmute(interfaceindex)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertInterfaceLuidToNameA(interfaceluid: *const NET_LUID_LH, interfacename: &mut [u8]) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertInterfaceLuidToNameA(interfaceluid: *const NET_LUID_LH, interfacename: ::windows::core::PSTR, length: usize) -> super::super::Foundation::NTSTATUS;
        }
        ConvertInterfaceLuidToNameA(::core::mem::transmute(interfaceluid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(interfacename)), interfacename.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertInterfaceLuidToNameW(interfaceluid: *const NET_LUID_LH, interfacename: &mut [u16]) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertInterfaceLuidToNameW(interfaceluid: *const NET_LUID_LH, interfacename: ::windows::core::PWSTR, length: usize) -> super::super::Foundation::NTSTATUS;
        }
        ConvertInterfaceLuidToNameW(::core::mem::transmute(interfaceluid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(interfacename)), interfacename.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertInterfaceNameToLuidA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(interfacename: Param0, interfaceluid: *mut NET_LUID_LH) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertInterfaceNameToLuidA(interfacename: ::windows::core::PCSTR, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
        }
        ConvertInterfaceNameToLuidA(interfacename.into_param().abi(), ::core::mem::transmute(interfaceluid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertInterfaceNameToLuidW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(interfacename: Param0, interfaceluid: *mut NET_LUID_LH) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertInterfaceNameToLuidW(interfacename: ::windows::core::PCWSTR, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
        }
        ConvertInterfaceNameToLuidW(interfacename.into_param().abi(), ::core::mem::transmute(interfaceluid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertIpv4MaskToLength(mask: u32, masklength: *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertIpv4MaskToLength(mask: u32, masklength: *mut u8) -> super::super::Foundation::NTSTATUS;
        }
        ConvertIpv4MaskToLength(::core::mem::transmute(mask), ::core::mem::transmute(masklength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertLengthToIpv4Mask(masklength: u32, mask: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertLengthToIpv4Mask(masklength: u32, mask: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        ConvertLengthToIpv4Mask(::core::mem::transmute(masklength), ::core::mem::transmute(mask)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn CreateAnycastIpAddressEntry(row: *const MIB_ANYCASTIPADDRESS_ROW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAnycastIpAddressEntry(row: *const MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
        }
        CreateAnycastIpAddressEntry(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn CreateIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32;
        }
        ::core::mem::transmute(CreateIpForwardEntry(::core::mem::transmute(proute)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn CreateIpForwardEntry2(row: *const MIB_IPFORWARD_ROW2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIpForwardEntry2(row: *const MIB_IPFORWARD_ROW2) -> super::super::Foundation::NTSTATUS;
        }
        CreateIpForwardEntry2(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn CreateIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32;
        }
        ::core::mem::transmute(CreateIpNetEntry(::core::mem::transmute(parpentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn CreateIpNetEntry2(row: *const MIB_IPNET_ROW2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::super::Foundation::NTSTATUS;
        }
        CreateIpNetEntry2(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn CreatePersistentTcpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePersistentTcpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32;
        }
        ::core::mem::transmute(CreatePersistentTcpPortReservation(::core::mem::transmute(startport), ::core::mem::transmute(numberofports), ::core::mem::transmute(token)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn CreatePersistentUdpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePersistentUdpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32;
        }
        ::core::mem::transmute(CreatePersistentUdpPortReservation(::core::mem::transmute(startport), ::core::mem::transmute(numberofports), ::core::mem::transmute(token)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn CreateProxyArpEntry(dwaddress: u32, dwmask: u32, dwifindex: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProxyArpEntry(dwaddress: u32, dwmask: u32, dwifindex: u32) -> u32;
        }
        ::core::mem::transmute(CreateProxyArpEntry(::core::mem::transmute(dwaddress), ::core::mem::transmute(dwmask), ::core::mem::transmute(dwifindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn CreateSortedAddressPairs(sourceaddresslist: *const super::super::Networking::WinSock::SOCKADDR_IN6, sourceaddresscount: u32, destinationaddresslist: *const super::super::Networking::WinSock::SOCKADDR_IN6, destinationaddresscount: u32, addresssortoptions: u32, sortedaddresspairlist: *mut *mut super::super::Networking::WinSock::SOCKADDR_IN6_PAIR, sortedaddresspaircount: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSortedAddressPairs(sourceaddresslist: *const super::super::Networking::WinSock::SOCKADDR_IN6, sourceaddresscount: u32, destinationaddresslist: *const super::super::Networking::WinSock::SOCKADDR_IN6, destinationaddresscount: u32, addresssortoptions: u32, sortedaddresspairlist: *mut *mut super::super::Networking::WinSock::SOCKADDR_IN6_PAIR, sortedaddresspaircount: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        CreateSortedAddressPairs(::core::mem::transmute(sourceaddresslist), ::core::mem::transmute(sourceaddresscount), ::core::mem::transmute(destinationaddresslist), ::core::mem::transmute(destinationaddresscount), ::core::mem::transmute(addresssortoptions), ::core::mem::transmute(sortedaddresspairlist), ::core::mem::transmute(sortedaddresspaircount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn CreateUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
        }
        CreateUnicastIpAddressEntry(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DEFAULT_MINIMUM_ENTITIES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DEST_LONGER: u32 = 29u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DEST_MATCHING: u32 = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DEST_SHORTER: u32 = 30u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_AUTO_UPGRADE_SERVER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_POLICY_AUTO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_POLICY_DISABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_POLICY_NOT_CONFIGURED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_POLICY_REQUIRED: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_DOH_SERVER_SETTINGS {
    pub Template: ::windows::core::PWSTR,
    pub Flags: u64,
}
impl ::core::marker::Copy for DNS_DOH_SERVER_SETTINGS {}
impl ::core::clone::Clone for DNS_DOH_SERVER_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DNS_DOH_SERVER_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_DOH_SERVER_SETTINGS").field("Template", &self.Template).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for DNS_DOH_SERVER_SETTINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DNS_DOH_SERVER_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DNS_DOH_SERVER_SETTINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DNS_DOH_SERVER_SETTINGS {}
impl ::core::default::Default for DNS_DOH_SERVER_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_SERVER_SETTINGS_ENABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_SERVER_SETTINGS_ENABLE_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_SERVER_SETTINGS_FALLBACK_TO_UDP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_ENABLE_DOH: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_INTERFACE_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
    pub Domain: ::windows::core::PWSTR,
    pub NameServer: ::windows::core::PWSTR,
    pub SearchList: ::windows::core::PWSTR,
    pub RegistrationEnabled: u32,
    pub RegisterAdapterName: u32,
    pub EnableLLMNR: u32,
    pub QueryAdapterName: u32,
    pub ProfileNameServer: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DNS_INTERFACE_SETTINGS {}
impl ::core::clone::Clone for DNS_INTERFACE_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DNS_INTERFACE_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_INTERFACE_SETTINGS").field("Version", &self.Version).field("Flags", &self.Flags).field("Domain", &self.Domain).field("NameServer", &self.NameServer).field("SearchList", &self.SearchList).field("RegistrationEnabled", &self.RegistrationEnabled).field("RegisterAdapterName", &self.RegisterAdapterName).field("EnableLLMNR", &self.EnableLLMNR).field("QueryAdapterName", &self.QueryAdapterName).field("ProfileNameServer", &self.ProfileNameServer).finish()
    }
}
unsafe impl ::windows::core::Abi for DNS_INTERFACE_SETTINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DNS_INTERFACE_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DNS_INTERFACE_SETTINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DNS_INTERFACE_SETTINGS {}
impl ::core::default::Default for DNS_INTERFACE_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_INTERFACE_SETTINGS3 {
    pub Version: u32,
    pub Flags: u64,
    pub Domain: ::windows::core::PWSTR,
    pub NameServer: ::windows::core::PWSTR,
    pub SearchList: ::windows::core::PWSTR,
    pub RegistrationEnabled: u32,
    pub RegisterAdapterName: u32,
    pub EnableLLMNR: u32,
    pub QueryAdapterName: u32,
    pub ProfileNameServer: ::windows::core::PWSTR,
    pub DisableUnconstrainedQueries: u32,
    pub SupplementalSearchList: ::windows::core::PWSTR,
    pub cServerProperties: u32,
    pub ServerProperties: *mut DNS_SERVER_PROPERTY,
    pub cProfileServerProperties: u32,
    pub ProfileServerProperties: *mut DNS_SERVER_PROPERTY,
}
impl ::core::marker::Copy for DNS_INTERFACE_SETTINGS3 {}
impl ::core::clone::Clone for DNS_INTERFACE_SETTINGS3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DNS_INTERFACE_SETTINGS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_INTERFACE_SETTINGS3")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("Domain", &self.Domain)
            .field("NameServer", &self.NameServer)
            .field("SearchList", &self.SearchList)
            .field("RegistrationEnabled", &self.RegistrationEnabled)
            .field("RegisterAdapterName", &self.RegisterAdapterName)
            .field("EnableLLMNR", &self.EnableLLMNR)
            .field("QueryAdapterName", &self.QueryAdapterName)
            .field("ProfileNameServer", &self.ProfileNameServer)
            .field("DisableUnconstrainedQueries", &self.DisableUnconstrainedQueries)
            .field("SupplementalSearchList", &self.SupplementalSearchList)
            .field("cServerProperties", &self.cServerProperties)
            .field("ServerProperties", &self.ServerProperties)
            .field("cProfileServerProperties", &self.cProfileServerProperties)
            .field("ProfileServerProperties", &self.ProfileServerProperties)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DNS_INTERFACE_SETTINGS3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DNS_INTERFACE_SETTINGS3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DNS_INTERFACE_SETTINGS3>()) == 0 }
    }
}
impl ::core::cmp::Eq for DNS_INTERFACE_SETTINGS3 {}
impl ::core::default::Default for DNS_INTERFACE_SETTINGS3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_INTERFACE_SETTINGS_EX {
    pub SettingsV1: DNS_INTERFACE_SETTINGS,
    pub DisableUnconstrainedQueries: u32,
    pub SupplementalSearchList: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DNS_INTERFACE_SETTINGS_EX {}
impl ::core::clone::Clone for DNS_INTERFACE_SETTINGS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DNS_INTERFACE_SETTINGS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_INTERFACE_SETTINGS_EX").field("SettingsV1", &self.SettingsV1).field("DisableUnconstrainedQueries", &self.DisableUnconstrainedQueries).field("SupplementalSearchList", &self.SupplementalSearchList).finish()
    }
}
unsafe impl ::windows::core::Abi for DNS_INTERFACE_SETTINGS_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DNS_INTERFACE_SETTINGS_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DNS_INTERFACE_SETTINGS_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for DNS_INTERFACE_SETTINGS_EX {}
impl ::core::default::Default for DNS_INTERFACE_SETTINGS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_INTERFACE_SETTINGS_VERSION1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_INTERFACE_SETTINGS_VERSION2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_INTERFACE_SETTINGS_VERSION3: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_SERVER_PROPERTY {
    pub Version: u32,
    pub ServerIndex: u32,
    pub Type: DNS_SERVER_PROPERTY_TYPE,
    pub Property: DNS_SERVER_PROPERTY_TYPES,
}
impl ::core::marker::Copy for DNS_SERVER_PROPERTY {}
impl ::core::clone::Clone for DNS_SERVER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DNS_SERVER_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DNS_SERVER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DNS_SERVER_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DNS_SERVER_PROPERTY {}
impl ::core::default::Default for DNS_SERVER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DNS_SERVER_PROPERTY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DnsServerInvalidProperty: DNS_SERVER_PROPERTY_TYPE = DNS_SERVER_PROPERTY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DnsServerDohProperty: DNS_SERVER_PROPERTY_TYPE = DNS_SERVER_PROPERTY_TYPE(1i32);
impl ::core::marker::Copy for DNS_SERVER_PROPERTY_TYPE {}
impl ::core::clone::Clone for DNS_SERVER_PROPERTY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DNS_SERVER_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DNS_SERVER_PROPERTY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DNS_SERVER_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_SERVER_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union DNS_SERVER_PROPERTY_TYPES {
    pub DohSettings: *mut DNS_DOH_SERVER_SETTINGS,
}
impl ::core::marker::Copy for DNS_SERVER_PROPERTY_TYPES {}
impl ::core::clone::Clone for DNS_SERVER_PROPERTY_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DNS_SERVER_PROPERTY_TYPES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DNS_SERVER_PROPERTY_TYPES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DNS_SERVER_PROPERTY_TYPES>()) == 0 }
    }
}
impl ::core::cmp::Eq for DNS_SERVER_PROPERTY_TYPES {}
impl ::core::default::Default for DNS_SERVER_PROPERTY_TYPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SERVER_PROPERTY_VERSION1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
    pub Hostname: ::windows::core::PWSTR,
    pub Domain: ::windows::core::PWSTR,
    pub SearchList: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DNS_SETTINGS {}
impl ::core::clone::Clone for DNS_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DNS_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_SETTINGS").field("Version", &self.Version).field("Flags", &self.Flags).field("Hostname", &self.Hostname).field("Domain", &self.Domain).field("SearchList", &self.SearchList).finish()
    }
}
unsafe impl ::windows::core::Abi for DNS_SETTINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DNS_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DNS_SETTINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DNS_SETTINGS {}
impl ::core::default::Default for DNS_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_SETTINGS2 {
    pub Version: u32,
    pub Flags: u64,
    pub Hostname: ::windows::core::PWSTR,
    pub Domain: ::windows::core::PWSTR,
    pub SearchList: ::windows::core::PWSTR,
    pub SettingFlags: u64,
}
impl ::core::marker::Copy for DNS_SETTINGS2 {}
impl ::core::clone::Clone for DNS_SETTINGS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DNS_SETTINGS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_SETTINGS2").field("Version", &self.Version).field("Flags", &self.Flags).field("Hostname", &self.Hostname).field("Domain", &self.Domain).field("SearchList", &self.SearchList).field("SettingFlags", &self.SettingFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for DNS_SETTINGS2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DNS_SETTINGS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DNS_SETTINGS2>()) == 0 }
    }
}
impl ::core::cmp::Eq for DNS_SETTINGS2 {}
impl ::core::default::Default for DNS_SETTINGS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTINGS_ENABLE_LLMNR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTINGS_QUERY_ADAPTER_NAME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTINGS_VERSION1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTINGS_VERSION2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_DISABLE_UNCONSTRAINED_QUERIES: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_DOH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_DOH_PROFILE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_DOMAIN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_HOSTNAME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_IPV6: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_NAMESERVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_PROFILE_NAMESERVER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_REGISTER_ADAPTER_NAME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_REGISTRATION_ENABLED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_SEARCHLIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_SUPPLEMENTAL_SEARCH_LIST: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DeleteAnycastIpAddressEntry(row: *const MIB_ANYCASTIPADDRESS_ROW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteAnycastIpAddressEntry(row: *const MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
        }
        DeleteAnycastIpAddressEntry(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn DeleteIPAddress(ntecontext: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteIPAddress(ntecontext: u32) -> u32;
        }
        ::core::mem::transmute(DeleteIPAddress(::core::mem::transmute(ntecontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DeleteIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32;
        }
        ::core::mem::transmute(DeleteIpForwardEntry(::core::mem::transmute(proute)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DeleteIpForwardEntry2(row: *const MIB_IPFORWARD_ROW2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteIpForwardEntry2(row: *const MIB_IPFORWARD_ROW2) -> super::super::Foundation::NTSTATUS;
        }
        DeleteIpForwardEntry2(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn DeleteIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32;
        }
        ::core::mem::transmute(DeleteIpNetEntry(::core::mem::transmute(parpentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DeleteIpNetEntry2(row: *const MIB_IPNET_ROW2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::super::Foundation::NTSTATUS;
        }
        DeleteIpNetEntry2(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn DeletePersistentTcpPortReservation(startport: u16, numberofports: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeletePersistentTcpPortReservation(startport: u16, numberofports: u16) -> u32;
        }
        ::core::mem::transmute(DeletePersistentTcpPortReservation(::core::mem::transmute(startport), ::core::mem::transmute(numberofports)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn DeletePersistentUdpPortReservation(startport: u16, numberofports: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeletePersistentUdpPortReservation(startport: u16, numberofports: u16) -> u32;
        }
        ::core::mem::transmute(DeletePersistentUdpPortReservation(::core::mem::transmute(startport), ::core::mem::transmute(numberofports)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn DeleteProxyArpEntry(dwaddress: u32, dwmask: u32, dwifindex: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteProxyArpEntry(dwaddress: u32, dwmask: u32, dwifindex: u32) -> u32;
        }
        ::core::mem::transmute(DeleteProxyArpEntry(::core::mem::transmute(dwaddress), ::core::mem::transmute(dwmask), ::core::mem::transmute(dwifindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DeleteUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
        }
        DeleteUnicastIpAddressEntry(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn DisableMediaSense(phandle: *mut super::super::Foundation::HANDLE, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisableMediaSense(phandle: *mut super::super::Foundation::HANDLE, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(DisableMediaSense(::core::mem::transmute(phandle), ::core::mem::transmute(poverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ERROR_BASE: u32 = 23000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ERROR_IPV6_NOT_IMPLEMENTED: u32 = 23003u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn EnableRouter(phandle: *mut super::super::Foundation::HANDLE, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableRouter(phandle: *mut super::super::Foundation::HANDLE, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(EnableRouter(::core::mem::transmute(phandle), ::core::mem::transmute(poverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const FD_FLAGS_ALLFLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const FD_FLAGS_NOSYN: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FIXED_INFO_W2KSP1 {
    pub HostName: [super::super::Foundation::CHAR; 132],
    pub DomainName: [super::super::Foundation::CHAR; 132],
    pub CurrentDnsServer: *mut IP_ADDR_STRING,
    pub DnsServerList: IP_ADDR_STRING,
    pub NodeType: u32,
    pub ScopeId: [super::super::Foundation::CHAR; 260],
    pub EnableRouting: u32,
    pub EnableProxy: u32,
    pub EnableDns: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FIXED_INFO_W2KSP1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FIXED_INFO_W2KSP1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FIXED_INFO_W2KSP1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIXED_INFO_W2KSP1").field("HostName", &self.HostName).field("DomainName", &self.DomainName).field("CurrentDnsServer", &self.CurrentDnsServer).field("DnsServerList", &self.DnsServerList).field("NodeType", &self.NodeType).field("ScopeId", &self.ScopeId).field("EnableRouting", &self.EnableRouting).field("EnableProxy", &self.EnableProxy).field("EnableDns", &self.EnableDns).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FIXED_INFO_W2KSP1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FIXED_INFO_W2KSP1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FIXED_INFO_W2KSP1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FIXED_INFO_W2KSP1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FIXED_INFO_W2KSP1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn FlushIpNetTable(dwifindex: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushIpNetTable(dwifindex: u32) -> u32;
        }
        ::core::mem::transmute(FlushIpNetTable(::core::mem::transmute(dwifindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushIpNetTable2(family: u16, interfaceindex: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushIpNetTable2(family: u16, interfaceindex: u32) -> super::super::Foundation::NTSTATUS;
        }
        FlushIpNetTable2(::core::mem::transmute(family), ::core::mem::transmute(interfaceindex)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushIpPathTable(family: u16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushIpPathTable(family: u16) -> super::super::Foundation::NTSTATUS;
        }
        FlushIpPathTable(::core::mem::transmute(family)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn FreeDnsSettings(settings: *mut DNS_SETTINGS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeDnsSettings(settings: *mut DNS_SETTINGS);
        }
        FreeDnsSettings(::core::mem::transmute(settings))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn FreeInterfaceDnsSettings(settings: *mut DNS_INTERFACE_SETTINGS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeInterfaceDnsSettings(settings: *mut DNS_INTERFACE_SETTINGS);
        }
        FreeInterfaceDnsSettings(::core::mem::transmute(settings))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn FreeMibTable(memory: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeMibTable(memory: *const ::core::ffi::c_void);
        }
        FreeMibTable(::core::mem::transmute(memory))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_SKIP_DNS_INFO: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_ADAPTERS_ADDRESSES_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_SKIP_UNICAST: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_SKIP_ANYCAST: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_SKIP_MULTICAST: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_SKIP_DNS_SERVER: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_INCLUDE_PREFIX: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_SKIP_FRIENDLY_NAME: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_INCLUDE_WINS_INFO: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_INCLUDE_GATEWAYS: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_INCLUDE_ALL_INTERFACES: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_INCLUDE_ALL_COMPARTMENTS: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_INCLUDE_TUNNEL_BINDINGORDER: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(1024u32);
impl ::core::marker::Copy for GET_ADAPTERS_ADDRESSES_FLAGS {}
impl ::core::clone::Clone for GET_ADAPTERS_ADDRESSES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_ADAPTERS_ADDRESSES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GET_ADAPTERS_ADDRESSES_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_ADAPTERS_ADDRESSES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_ADAPTERS_ADDRESSES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GET_ADAPTERS_ADDRESSES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_ADAPTERS_ADDRESSES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_ADAPTERS_ADDRESSES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_ADAPTERS_ADDRESSES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_ADAPTERS_ADDRESSES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBAL_FILTER(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GF_FRAGMENTS: GLOBAL_FILTER = GLOBAL_FILTER(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GF_STRONGHOST: GLOBAL_FILTER = GLOBAL_FILTER(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GF_FRAGCACHE: GLOBAL_FILTER = GLOBAL_FILTER(9i32);
impl ::core::marker::Copy for GLOBAL_FILTER {}
impl ::core::clone::Clone for GLOBAL_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBAL_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GLOBAL_FILTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBAL_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBAL_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetAdapterIndex<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(adaptername: Param0, ifindex: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAdapterIndex(adaptername: ::windows::core::PCWSTR, ifindex: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetAdapterIndex(adaptername.into_param().abi(), ::core::mem::transmute(ifindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetAdapterOrderMap() -> *mut IP_ADAPTER_ORDER_MAP {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAdapterOrderMap() -> *mut IP_ADAPTER_ORDER_MAP;
        }
        ::core::mem::transmute(GetAdapterOrderMap())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetAdaptersAddresses(family: ADDRESS_FAMILY, flags: GET_ADAPTERS_ADDRESSES_FLAGS, reserved: *mut ::core::ffi::c_void, adapteraddresses: *mut IP_ADAPTER_ADDRESSES_LH, sizepointer: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAdaptersAddresses(family: ADDRESS_FAMILY, flags: GET_ADAPTERS_ADDRESSES_FLAGS, reserved: *mut ::core::ffi::c_void, adapteraddresses: *mut IP_ADAPTER_ADDRESSES_LH, sizepointer: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetAdaptersAddresses(::core::mem::transmute(family), ::core::mem::transmute(flags), ::core::mem::transmute(reserved), ::core::mem::transmute(adapteraddresses), ::core::mem::transmute(sizepointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAdaptersInfo(adapterinfo: *mut IP_ADAPTER_INFO, sizepointer: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAdaptersInfo(adapterinfo: *mut IP_ADAPTER_INFO, sizepointer: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetAdaptersInfo(::core::mem::transmute(adapterinfo), ::core::mem::transmute(sizepointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetAnycastIpAddressEntry(row: *mut MIB_ANYCASTIPADDRESS_ROW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAnycastIpAddressEntry(row: *mut MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
        }
        GetAnycastIpAddressEntry(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetAnycastIpAddressTable(family: u16, table: *mut *mut MIB_ANYCASTIPADDRESS_TABLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAnycastIpAddressTable(family: u16, table: *mut *mut MIB_ANYCASTIPADDRESS_TABLE) -> super::super::Foundation::NTSTATUS;
        }
        GetAnycastIpAddressTable(::core::mem::transmute(family), ::core::mem::transmute(table)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetBestInterface(dwdestaddr: u32, pdwbestifindex: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBestInterface(dwdestaddr: u32, pdwbestifindex: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetBestInterface(::core::mem::transmute(dwdestaddr), ::core::mem::transmute(pdwbestifindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetBestInterfaceEx(pdestaddr: *const super::super::Networking::WinSock::SOCKADDR, pdwbestifindex: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBestInterfaceEx(pdestaddr: *const super::super::Networking::WinSock::SOCKADDR, pdwbestifindex: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetBestInterfaceEx(::core::mem::transmute(pdestaddr), ::core::mem::transmute(pdwbestifindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn GetBestRoute(dwdestaddr: u32, dwsourceaddr: u32, pbestroute: *mut MIB_IPFORWARDROW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBestRoute(dwdestaddr: u32, dwsourceaddr: u32, pbestroute: *mut MIB_IPFORWARDROW) -> u32;
        }
        ::core::mem::transmute(GetBestRoute(::core::mem::transmute(dwdestaddr), ::core::mem::transmute(dwsourceaddr), ::core::mem::transmute(pbestroute)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetBestRoute2(interfaceluid: *const NET_LUID_LH, interfaceindex: u32, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_INET, destinationaddress: *const super::super::Networking::WinSock::SOCKADDR_INET, addresssortoptions: u32, bestroute: *mut MIB_IPFORWARD_ROW2, bestsourceaddress: *mut super::super::Networking::WinSock::SOCKADDR_INET) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBestRoute2(interfaceluid: *const NET_LUID_LH, interfaceindex: u32, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_INET, destinationaddress: *const super::super::Networking::WinSock::SOCKADDR_INET, addresssortoptions: u32, bestroute: *mut MIB_IPFORWARD_ROW2, bestsourceaddress: *mut super::super::Networking::WinSock::SOCKADDR_INET) -> super::super::Foundation::NTSTATUS;
        }
        GetBestRoute2(::core::mem::transmute(interfaceluid), ::core::mem::transmute(interfaceindex), ::core::mem::transmute(sourceaddress), ::core::mem::transmute(destinationaddress), ::core::mem::transmute(addresssortoptions), ::core::mem::transmute(bestroute), ::core::mem::transmute(bestsourceaddress)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetCurrentThreadCompartmentId() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentThreadCompartmentId() -> u32;
        }
        ::core::mem::transmute(GetCurrentThreadCompartmentId())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetCurrentThreadCompartmentScope(compartmentscope: *mut u32, compartmentid: *mut u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentThreadCompartmentScope(compartmentscope: *mut u32, compartmentid: *mut u32);
        }
        GetCurrentThreadCompartmentScope(::core::mem::transmute(compartmentscope), ::core::mem::transmute(compartmentid))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetDefaultCompartmentId() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDefaultCompartmentId() -> u32;
        }
        ::core::mem::transmute(GetDefaultCompartmentId())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDnsSettings(settings: *mut DNS_SETTINGS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDnsSettings(settings: *mut DNS_SETTINGS) -> super::super::Foundation::NTSTATUS;
        }
        GetDnsSettings(::core::mem::transmute(settings)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetExtendedTcpTable<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(ptcptable: *mut ::core::ffi::c_void, pdwsize: *mut u32, border: Param2, ulaf: u32, tableclass: TCP_TABLE_CLASS, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExtendedTcpTable(ptcptable: *mut ::core::ffi::c_void, pdwsize: *mut u32, border: super::super::Foundation::BOOL, ulaf: u32, tableclass: TCP_TABLE_CLASS, reserved: u32) -> u32;
        }
        ::core::mem::transmute(GetExtendedTcpTable(::core::mem::transmute(ptcptable), ::core::mem::transmute(pdwsize), border.into_param().abi(), ::core::mem::transmute(ulaf), ::core::mem::transmute(tableclass), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetExtendedUdpTable<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pudptable: *mut ::core::ffi::c_void, pdwsize: *mut u32, border: Param2, ulaf: u32, tableclass: UDP_TABLE_CLASS, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExtendedUdpTable(pudptable: *mut ::core::ffi::c_void, pdwsize: *mut u32, border: super::super::Foundation::BOOL, ulaf: u32, tableclass: UDP_TABLE_CLASS, reserved: u32) -> u32;
        }
        ::core::mem::transmute(GetExtendedUdpTable(::core::mem::transmute(pudptable), ::core::mem::transmute(pdwsize), border.into_param().abi(), ::core::mem::transmute(ulaf), ::core::mem::transmute(tableclass), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetFriendlyIfIndex(ifindex: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFriendlyIfIndex(ifindex: u32) -> u32;
        }
        ::core::mem::transmute(GetFriendlyIfIndex(::core::mem::transmute(ifindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetIcmpStatistics(statistics: *mut MIB_ICMP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIcmpStatistics(statistics: *mut MIB_ICMP) -> u32;
        }
        ::core::mem::transmute(GetIcmpStatistics(::core::mem::transmute(statistics)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetIcmpStatisticsEx(statistics: *mut MIB_ICMP_EX_XPSP1, family: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIcmpStatisticsEx(statistics: *mut MIB_ICMP_EX_XPSP1, family: u32) -> u32;
        }
        ::core::mem::transmute(GetIcmpStatisticsEx(::core::mem::transmute(statistics), ::core::mem::transmute(family)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetIfEntry(pifrow: *mut MIB_IFROW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIfEntry(pifrow: *mut MIB_IFROW) -> u32;
        }
        ::core::mem::transmute(GetIfEntry(::core::mem::transmute(pifrow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[inline]
pub unsafe fn GetIfEntry2(row: *mut MIB_IF_ROW2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIfEntry2(row: *mut MIB_IF_ROW2) -> super::super::Foundation::NTSTATUS;
        }
        GetIfEntry2(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[inline]
pub unsafe fn GetIfEntry2Ex(level: MIB_IF_ENTRY_LEVEL, row: *mut MIB_IF_ROW2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIfEntry2Ex(level: MIB_IF_ENTRY_LEVEL, row: *mut MIB_IF_ROW2) -> super::super::Foundation::NTSTATUS;
        }
        GetIfEntry2Ex(::core::mem::transmute(level), ::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetIfStackTable(table: *mut *mut MIB_IFSTACK_TABLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIfStackTable(table: *mut *mut MIB_IFSTACK_TABLE) -> super::super::Foundation::NTSTATUS;
        }
        GetIfStackTable(::core::mem::transmute(table)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetIfTable<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(piftable: *mut MIB_IFTABLE, pdwsize: *mut u32, border: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIfTable(piftable: *mut MIB_IFTABLE, pdwsize: *mut u32, border: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(GetIfTable(::core::mem::transmute(piftable), ::core::mem::transmute(pdwsize), border.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[inline]
pub unsafe fn GetIfTable2(table: *mut *mut MIB_IF_TABLE2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIfTable2(table: *mut *mut MIB_IF_TABLE2) -> super::super::Foundation::NTSTATUS;
        }
        GetIfTable2(::core::mem::transmute(table)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[inline]
pub unsafe fn GetIfTable2Ex(level: MIB_IF_TABLE_LEVEL, table: *mut *mut MIB_IF_TABLE2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIfTable2Ex(level: MIB_IF_TABLE_LEVEL, table: *mut *mut MIB_IF_TABLE2) -> super::super::Foundation::NTSTATUS;
        }
        GetIfTable2Ex(::core::mem::transmute(level), ::core::mem::transmute(table)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetInterfaceActiveTimestampCapabilities(interfaceluid: *const NET_LUID_LH, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetInterfaceActiveTimestampCapabilities(interfaceluid: *const NET_LUID_LH, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32;
        }
        ::core::mem::transmute(GetInterfaceActiveTimestampCapabilities(::core::mem::transmute(interfaceluid), ::core::mem::transmute(timestampcapabilites)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetInterfaceDnsSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(interface: Param0, settings: *mut DNS_INTERFACE_SETTINGS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetInterfaceDnsSettings(interface: ::windows::core::GUID, settings: *mut DNS_INTERFACE_SETTINGS) -> super::super::Foundation::NTSTATUS;
        }
        GetInterfaceDnsSettings(interface.into_param().abi(), ::core::mem::transmute(settings)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetInterfaceInfo(piftable: *mut IP_INTERFACE_INFO, dwoutbuflen: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetInterfaceInfo(piftable: *mut IP_INTERFACE_INFO, dwoutbuflen: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetInterfaceInfo(::core::mem::transmute(piftable), ::core::mem::transmute(dwoutbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetInterfaceSupportedTimestampCapabilities(interfaceluid: *const NET_LUID_LH, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetInterfaceSupportedTimestampCapabilities(interfaceluid: *const NET_LUID_LH, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32;
        }
        ::core::mem::transmute(GetInterfaceSupportedTimestampCapabilities(::core::mem::transmute(interfaceluid), ::core::mem::transmute(timestampcapabilites)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetInvertedIfStackTable(table: *mut *mut MIB_INVERTEDIFSTACK_TABLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetInvertedIfStackTable(table: *mut *mut MIB_INVERTEDIFSTACK_TABLE) -> super::super::Foundation::NTSTATUS;
        }
        GetInvertedIfStackTable(::core::mem::transmute(table)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetIpAddrTable<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pipaddrtable: *mut MIB_IPADDRTABLE, pdwsize: *mut u32, border: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpAddrTable(pipaddrtable: *mut MIB_IPADDRTABLE, pdwsize: *mut u32, border: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(GetIpAddrTable(::core::mem::transmute(pipaddrtable), ::core::mem::transmute(pdwsize), border.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetIpErrorString(errorcode: u32, buffer: ::windows::core::PWSTR, size: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpErrorString(errorcode: u32, buffer: ::windows::core::PWSTR, size: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetIpErrorString(::core::mem::transmute(errorcode), ::core::mem::transmute(buffer), ::core::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpForwardEntry2(row: *mut MIB_IPFORWARD_ROW2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpForwardEntry2(row: *mut MIB_IPFORWARD_ROW2) -> super::super::Foundation::NTSTATUS;
        }
        GetIpForwardEntry2(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpForwardTable<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pipforwardtable: *mut MIB_IPFORWARDTABLE, pdwsize: *mut u32, border: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpForwardTable(pipforwardtable: *mut MIB_IPFORWARDTABLE, pdwsize: *mut u32, border: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(GetIpForwardTable(::core::mem::transmute(pipforwardtable), ::core::mem::transmute(pdwsize), border.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpForwardTable2(family: u16, table: *mut *mut MIB_IPFORWARD_TABLE2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpForwardTable2(family: u16, table: *mut *mut MIB_IPFORWARD_TABLE2) -> super::super::Foundation::NTSTATUS;
        }
        GetIpForwardTable2(::core::mem::transmute(family), ::core::mem::transmute(table)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) -> super::super::Foundation::NTSTATUS;
        }
        GetIpInterfaceEntry(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpInterfaceTable(family: u16, table: *mut *mut MIB_IPINTERFACE_TABLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpInterfaceTable(family: u16, table: *mut *mut MIB_IPINTERFACE_TABLE) -> super::super::Foundation::NTSTATUS;
        }
        GetIpInterfaceTable(::core::mem::transmute(family), ::core::mem::transmute(table)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpNetEntry2(row: *mut MIB_IPNET_ROW2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpNetEntry2(row: *mut MIB_IPNET_ROW2) -> super::super::Foundation::NTSTATUS;
        }
        GetIpNetEntry2(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetIpNetTable<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(ipnettable: *mut MIB_IPNETTABLE, sizepointer: *mut u32, order: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpNetTable(ipnettable: *mut MIB_IPNETTABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(GetIpNetTable(::core::mem::transmute(ipnettable), ::core::mem::transmute(sizepointer), order.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpNetTable2(family: u16, table: *mut *mut MIB_IPNET_TABLE2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpNetTable2(family: u16, table: *mut *mut MIB_IPNET_TABLE2) -> super::super::Foundation::NTSTATUS;
        }
        GetIpNetTable2(::core::mem::transmute(family), ::core::mem::transmute(table)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpNetworkConnectionBandwidthEstimates(interfaceindex: u32, addressfamily: u16, bandwidthestimates: *mut MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpNetworkConnectionBandwidthEstimates(interfaceindex: u32, addressfamily: u16, bandwidthestimates: *mut MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES) -> super::super::Foundation::NTSTATUS;
        }
        GetIpNetworkConnectionBandwidthEstimates(::core::mem::transmute(interfaceindex), ::core::mem::transmute(addressfamily), ::core::mem::transmute(bandwidthestimates)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpPathEntry(row: *mut MIB_IPPATH_ROW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpPathEntry(row: *mut MIB_IPPATH_ROW) -> super::super::Foundation::NTSTATUS;
        }
        GetIpPathEntry(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpPathTable(family: u16, table: *mut *mut MIB_IPPATH_TABLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpPathTable(family: u16, table: *mut *mut MIB_IPPATH_TABLE) -> super::super::Foundation::NTSTATUS;
        }
        GetIpPathTable(::core::mem::transmute(family), ::core::mem::transmute(table)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetIpStatistics(statistics: *mut MIB_IPSTATS_LH) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpStatistics(statistics: *mut MIB_IPSTATS_LH) -> u32;
        }
        ::core::mem::transmute(GetIpStatistics(::core::mem::transmute(statistics)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetIpStatisticsEx(statistics: *mut MIB_IPSTATS_LH, family: ADDRESS_FAMILY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIpStatisticsEx(statistics: *mut MIB_IPSTATS_LH, family: ADDRESS_FAMILY) -> u32;
        }
        ::core::mem::transmute(GetIpStatisticsEx(::core::mem::transmute(statistics), ::core::mem::transmute(family)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetJobCompartmentId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(jobhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetJobCompartmentId(jobhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(GetJobCompartmentId(jobhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetMulticastIpAddressEntry(row: *mut MIB_MULTICASTIPADDRESS_ROW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMulticastIpAddressEntry(row: *mut MIB_MULTICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
        }
        GetMulticastIpAddressEntry(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetMulticastIpAddressTable(family: u16, table: *mut *mut MIB_MULTICASTIPADDRESS_TABLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMulticastIpAddressTable(family: u16, table: *mut *mut MIB_MULTICASTIPADDRESS_TABLE) -> super::super::Foundation::NTSTATUS;
        }
        GetMulticastIpAddressTable(::core::mem::transmute(family), ::core::mem::transmute(table)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetNetworkConnectivityHint(connectivityhint: *mut super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNetworkConnectivityHint(connectivityhint: *mut super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT) -> super::super::Foundation::NTSTATUS;
        }
        GetNetworkConnectivityHint(::core::mem::transmute(connectivityhint)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetNetworkConnectivityHintForInterface(interfaceindex: u32, connectivityhint: *mut super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNetworkConnectivityHintForInterface(interfaceindex: u32, connectivityhint: *mut super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT) -> super::super::Foundation::NTSTATUS;
        }
        GetNetworkConnectivityHintForInterface(::core::mem::transmute(interfaceindex), ::core::mem::transmute(connectivityhint)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNetworkInformation(networkguid: *const ::windows::core::GUID, compartmentid: *mut u32, siteid: *mut u32, networkname: &mut [u16]) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNetworkInformation(networkguid: *const ::windows::core::GUID, compartmentid: *mut u32, siteid: *mut u32, networkname: ::windows::core::PWSTR, length: u32) -> super::super::Foundation::NTSTATUS;
        }
        GetNetworkInformation(::core::mem::transmute(networkguid), ::core::mem::transmute(compartmentid), ::core::mem::transmute(siteid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(networkname)), networkname.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNetworkParams(pfixedinfo: *mut FIXED_INFO_W2KSP1, poutbuflen: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNetworkParams(pfixedinfo: *mut FIXED_INFO_W2KSP1, poutbuflen: *mut u32) -> super::super::Foundation::WIN32_ERROR;
        }
        ::core::mem::transmute(GetNetworkParams(::core::mem::transmute(pfixedinfo), ::core::mem::transmute(poutbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetNumberOfInterfaces(pdwnumif: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumberOfInterfaces(pdwnumif: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetNumberOfInterfaces(::core::mem::transmute(pdwnumif)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetOwnerModuleFromPidAndInfo(ulpid: u32, pinfo: *const u64, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOwnerModuleFromPidAndInfo(ulpid: u32, pinfo: *const u64, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetOwnerModuleFromPidAndInfo(::core::mem::transmute(ulpid), ::core::mem::transmute(pinfo), ::core::mem::transmute(class), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetOwnerModuleFromTcp6Entry(ptcpentry: *const MIB_TCP6ROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOwnerModuleFromTcp6Entry(ptcpentry: *const MIB_TCP6ROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetOwnerModuleFromTcp6Entry(::core::mem::transmute(ptcpentry), ::core::mem::transmute(class), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetOwnerModuleFromTcpEntry(ptcpentry: *const MIB_TCPROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOwnerModuleFromTcpEntry(ptcpentry: *const MIB_TCPROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetOwnerModuleFromTcpEntry(::core::mem::transmute(ptcpentry), ::core::mem::transmute(class), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetOwnerModuleFromUdp6Entry(pudpentry: *const MIB_UDP6ROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOwnerModuleFromUdp6Entry(pudpentry: *const MIB_UDP6ROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetOwnerModuleFromUdp6Entry(::core::mem::transmute(pudpentry), ::core::mem::transmute(class), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetOwnerModuleFromUdpEntry(pudpentry: *const MIB_UDPROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOwnerModuleFromUdpEntry(pudpentry: *const MIB_UDPROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetOwnerModuleFromUdpEntry(::core::mem::transmute(pudpentry), ::core::mem::transmute(class), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPerAdapterInfo(ifindex: u32, pperadapterinfo: *mut IP_PER_ADAPTER_INFO_W2KSP1, poutbuflen: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPerAdapterInfo(ifindex: u32, pperadapterinfo: *mut IP_PER_ADAPTER_INFO_W2KSP1, poutbuflen: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetPerAdapterInfo(::core::mem::transmute(ifindex), ::core::mem::transmute(pperadapterinfo), ::core::mem::transmute(poutbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn GetPerTcp6ConnectionEStats(row: *const MIB_TCP6ROW, estatstype: TCP_ESTATS_TYPE, rw: *mut u8, rwversion: u32, rwsize: u32, ros: *mut u8, rosversion: u32, rossize: u32, rod: *mut u8, rodversion: u32, rodsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPerTcp6ConnectionEStats(row: *const MIB_TCP6ROW, estatstype: TCP_ESTATS_TYPE, rw: *mut u8, rwversion: u32, rwsize: u32, ros: *mut u8, rosversion: u32, rossize: u32, rod: *mut u8, rodversion: u32, rodsize: u32) -> u32;
        }
        ::core::mem::transmute(GetPerTcp6ConnectionEStats(::core::mem::transmute(row), ::core::mem::transmute(estatstype), ::core::mem::transmute(rw), ::core::mem::transmute(rwversion), ::core::mem::transmute(rwsize), ::core::mem::transmute(ros), ::core::mem::transmute(rosversion), ::core::mem::transmute(rossize), ::core::mem::transmute(rod), ::core::mem::transmute(rodversion), ::core::mem::transmute(rodsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetPerTcpConnectionEStats(row: *const MIB_TCPROW_LH, estatstype: TCP_ESTATS_TYPE, rw: *mut u8, rwversion: u32, rwsize: u32, ros: *mut u8, rosversion: u32, rossize: u32, rod: *mut u8, rodversion: u32, rodsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPerTcpConnectionEStats(row: *const MIB_TCPROW_LH, estatstype: TCP_ESTATS_TYPE, rw: *mut u8, rwversion: u32, rwsize: u32, ros: *mut u8, rosversion: u32, rossize: u32, rod: *mut u8, rodversion: u32, rodsize: u32) -> u32;
        }
        ::core::mem::transmute(GetPerTcpConnectionEStats(::core::mem::transmute(row), ::core::mem::transmute(estatstype), ::core::mem::transmute(rw), ::core::mem::transmute(rwversion), ::core::mem::transmute(rwsize), ::core::mem::transmute(ros), ::core::mem::transmute(rosversion), ::core::mem::transmute(rossize), ::core::mem::transmute(rod), ::core::mem::transmute(rodversion), ::core::mem::transmute(rodsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRTTAndHopCount(destipaddress: u32, hopcount: *mut u32, maxhops: u32, rtt: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRTTAndHopCount(destipaddress: u32, hopcount: *mut u32, maxhops: u32, rtt: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetRTTAndHopCount(::core::mem::transmute(destipaddress), ::core::mem::transmute(hopcount), ::core::mem::transmute(maxhops), ::core::mem::transmute(rtt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetSessionCompartmentId(sessionid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSessionCompartmentId(sessionid: u32) -> u32;
        }
        ::core::mem::transmute(GetSessionCompartmentId(::core::mem::transmute(sessionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetTcp6Table<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(tcptable: *mut MIB_TCP6TABLE, sizepointer: *mut u32, order: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTcp6Table(tcptable: *mut MIB_TCP6TABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(GetTcp6Table(::core::mem::transmute(tcptable), ::core::mem::transmute(sizepointer), order.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetTcp6Table2<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(tcptable: *mut MIB_TCP6TABLE2, sizepointer: *mut u32, order: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTcp6Table2(tcptable: *mut MIB_TCP6TABLE2, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(GetTcp6Table2(::core::mem::transmute(tcptable), ::core::mem::transmute(sizepointer), order.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetTcpStatistics(statistics: *mut MIB_TCPSTATS_LH) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTcpStatistics(statistics: *mut MIB_TCPSTATS_LH) -> u32;
        }
        ::core::mem::transmute(GetTcpStatistics(::core::mem::transmute(statistics)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetTcpStatisticsEx(statistics: *mut MIB_TCPSTATS_LH, family: ADDRESS_FAMILY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTcpStatisticsEx(statistics: *mut MIB_TCPSTATS_LH, family: ADDRESS_FAMILY) -> u32;
        }
        ::core::mem::transmute(GetTcpStatisticsEx(::core::mem::transmute(statistics), ::core::mem::transmute(family)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetTcpStatisticsEx2(statistics: *mut MIB_TCPSTATS2, family: ADDRESS_FAMILY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTcpStatisticsEx2(statistics: *mut MIB_TCPSTATS2, family: ADDRESS_FAMILY) -> u32;
        }
        ::core::mem::transmute(GetTcpStatisticsEx2(::core::mem::transmute(statistics), ::core::mem::transmute(family)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTcpTable<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(tcptable: *mut MIB_TCPTABLE, sizepointer: *mut u32, order: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTcpTable(tcptable: *mut MIB_TCPTABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(GetTcpTable(::core::mem::transmute(tcptable), ::core::mem::transmute(sizepointer), order.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTcpTable2<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(tcptable: *mut MIB_TCPTABLE2, sizepointer: *mut u32, order: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTcpTable2(tcptable: *mut MIB_TCPTABLE2, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(GetTcpTable2(::core::mem::transmute(tcptable), ::core::mem::transmute(sizepointer), order.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTeredoPort(port: *mut u16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTeredoPort(port: *mut u16) -> super::super::Foundation::NTSTATUS;
        }
        GetTeredoPort(::core::mem::transmute(port)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetUdp6Table<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(udp6table: *mut MIB_UDP6TABLE, sizepointer: *mut u32, order: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUdp6Table(udp6table: *mut MIB_UDP6TABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(GetUdp6Table(::core::mem::transmute(udp6table), ::core::mem::transmute(sizepointer), order.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetUdpStatistics(stats: *mut MIB_UDPSTATS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUdpStatistics(stats: *mut MIB_UDPSTATS) -> u32;
        }
        ::core::mem::transmute(GetUdpStatistics(::core::mem::transmute(stats)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetUdpStatisticsEx(statistics: *mut MIB_UDPSTATS, family: ADDRESS_FAMILY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUdpStatisticsEx(statistics: *mut MIB_UDPSTATS, family: ADDRESS_FAMILY) -> u32;
        }
        ::core::mem::transmute(GetUdpStatisticsEx(::core::mem::transmute(statistics), ::core::mem::transmute(family)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetUdpStatisticsEx2(statistics: *mut MIB_UDPSTATS2, family: ADDRESS_FAMILY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUdpStatisticsEx2(statistics: *mut MIB_UDPSTATS2, family: ADDRESS_FAMILY) -> u32;
        }
        ::core::mem::transmute(GetUdpStatisticsEx2(::core::mem::transmute(statistics), ::core::mem::transmute(family)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUdpTable<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(udptable: *mut MIB_UDPTABLE, sizepointer: *mut u32, order: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUdpTable(udptable: *mut MIB_UDPTABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(GetUdpTable(::core::mem::transmute(udptable), ::core::mem::transmute(sizepointer), order.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn GetUniDirectionalAdapterInfo(pipifinfo: *mut IP_UNIDIRECTIONAL_ADAPTER_ADDRESS, dwoutbuflen: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUniDirectionalAdapterInfo(pipifinfo: *mut IP_UNIDIRECTIONAL_ADAPTER_ADDRESS, dwoutbuflen: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetUniDirectionalAdapterInfo(::core::mem::transmute(pipifinfo), ::core::mem::transmute(dwoutbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetUnicastIpAddressEntry(row: *mut MIB_UNICASTIPADDRESS_ROW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUnicastIpAddressEntry(row: *mut MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
        }
        GetUnicastIpAddressEntry(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetUnicastIpAddressTable(family: u16, table: *mut *mut MIB_UNICASTIPADDRESS_TABLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUnicastIpAddressTable(family: u16, table: *mut *mut MIB_UNICASTIPADDRESS_TABLE) -> super::super::Foundation::NTSTATUS;
        }
        GetUnicastIpAddressTable(::core::mem::transmute(family), ::core::mem::transmute(table)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HIFTIMESTAMPCHANGE(pub isize);
impl HIFTIMESTAMPCHANGE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HIFTIMESTAMPCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HIFTIMESTAMPCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HIFTIMESTAMPCHANGE {}
impl ::core::fmt::Debug for HIFTIMESTAMPCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIFTIMESTAMPCHANGE").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HIFTIMESTAMPCHANGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const HYBRID_NODETYPE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ICMP4_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_ECHO_REPLY: ICMP4_TYPE = ICMP4_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_DST_UNREACH: ICMP4_TYPE = ICMP4_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_SOURCE_QUENCH: ICMP4_TYPE = ICMP4_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_REDIRECT: ICMP4_TYPE = ICMP4_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_ECHO_REQUEST: ICMP4_TYPE = ICMP4_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_ROUTER_ADVERT: ICMP4_TYPE = ICMP4_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_ROUTER_SOLICIT: ICMP4_TYPE = ICMP4_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_TIME_EXCEEDED: ICMP4_TYPE = ICMP4_TYPE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_PARAM_PROB: ICMP4_TYPE = ICMP4_TYPE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_TIMESTAMP_REQUEST: ICMP4_TYPE = ICMP4_TYPE(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_TIMESTAMP_REPLY: ICMP4_TYPE = ICMP4_TYPE(14i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_MASK_REQUEST: ICMP4_TYPE = ICMP4_TYPE(17i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_MASK_REPLY: ICMP4_TYPE = ICMP4_TYPE(18i32);
impl ::core::marker::Copy for ICMP4_TYPE {}
impl ::core::clone::Clone for ICMP4_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ICMP4_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ICMP4_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ICMP4_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICMP4_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_INFOMSG_MASK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ICMP6_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_DST_UNREACH: ICMP6_TYPE = ICMP6_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_PACKET_TOO_BIG: ICMP6_TYPE = ICMP6_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_TIME_EXCEEDED: ICMP6_TYPE = ICMP6_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_PARAM_PROB: ICMP6_TYPE = ICMP6_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_ECHO_REQUEST: ICMP6_TYPE = ICMP6_TYPE(128i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_ECHO_REPLY: ICMP6_TYPE = ICMP6_TYPE(129i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_MEMBERSHIP_QUERY: ICMP6_TYPE = ICMP6_TYPE(130i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_MEMBERSHIP_REPORT: ICMP6_TYPE = ICMP6_TYPE(131i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_MEMBERSHIP_REDUCTION: ICMP6_TYPE = ICMP6_TYPE(132i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ND_ROUTER_SOLICIT: ICMP6_TYPE = ICMP6_TYPE(133i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ND_ROUTER_ADVERT: ICMP6_TYPE = ICMP6_TYPE(134i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ND_NEIGHBOR_SOLICIT: ICMP6_TYPE = ICMP6_TYPE(135i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ND_NEIGHBOR_ADVERT: ICMP6_TYPE = ICMP6_TYPE(136i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ND_REDIRECT: ICMP6_TYPE = ICMP6_TYPE(137i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_V2_MEMBERSHIP_REPORT: ICMP6_TYPE = ICMP6_TYPE(143i32);
impl ::core::marker::Copy for ICMP6_TYPE {}
impl ::core::clone::Clone for ICMP6_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ICMP6_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ICMP6_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ICMP6_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICMP6_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP_STATS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IF_ACCESS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ACCESS_LOOPBACK: IF_ACCESS_TYPE = IF_ACCESS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ACCESS_BROADCAST: IF_ACCESS_TYPE = IF_ACCESS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ACCESS_POINT_TO_POINT: IF_ACCESS_TYPE = IF_ACCESS_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ACCESS_POINTTOPOINT: IF_ACCESS_TYPE = IF_ACCESS_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ACCESS_POINT_TO_MULTI_POINT: IF_ACCESS_TYPE = IF_ACCESS_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ACCESS_POINTTOMULTIPOINT: IF_ACCESS_TYPE = IF_ACCESS_TYPE(4i32);
impl ::core::marker::Copy for IF_ACCESS_TYPE {}
impl ::core::clone::Clone for IF_ACCESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IF_ACCESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IF_ACCESS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IF_ACCESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IF_ACCESS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IF_ADMINISTRATIVE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ADMINISTRATIVE_DISABLED: IF_ADMINISTRATIVE_STATE = IF_ADMINISTRATIVE_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ADMINISTRATIVE_ENABLED: IF_ADMINISTRATIVE_STATE = IF_ADMINISTRATIVE_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ADMINISTRATIVE_DEMANDDIAL: IF_ADMINISTRATIVE_STATE = IF_ADMINISTRATIVE_STATE(2i32);
impl ::core::marker::Copy for IF_ADMINISTRATIVE_STATE {}
impl ::core::clone::Clone for IF_ADMINISTRATIVE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IF_ADMINISTRATIVE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IF_ADMINISTRATIVE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IF_ADMINISTRATIVE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IF_ADMINISTRATIVE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ADMIN_STATUS_DOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ADMIN_STATUS_TESTING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ADMIN_STATUS_UP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_CHECK_MCAST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_CHECK_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_CHECK_SEND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_CONNECTION_DEDICATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_CONNECTION_DEMAND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_CONNECTION_PASSIVE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IF_COUNTED_STRING_LH {
    pub Length: u16,
    pub String: [u16; 257],
}
impl ::core::marker::Copy for IF_COUNTED_STRING_LH {}
impl ::core::clone::Clone for IF_COUNTED_STRING_LH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IF_COUNTED_STRING_LH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IF_COUNTED_STRING_LH").field("Length", &self.Length).field("String", &self.String).finish()
    }
}
unsafe impl ::windows::core::Abi for IF_COUNTED_STRING_LH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IF_COUNTED_STRING_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IF_COUNTED_STRING_LH>()) == 0 }
    }
}
impl ::core::cmp::Eq for IF_COUNTED_STRING_LH {}
impl ::core::default::Default for IF_COUNTED_STRING_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_MAX_PHYS_ADDRESS_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_MAX_STRING_SIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_NUMBER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IF_OPER_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusUp: IF_OPER_STATUS = IF_OPER_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusDown: IF_OPER_STATUS = IF_OPER_STATUS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusTesting: IF_OPER_STATUS = IF_OPER_STATUS(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusUnknown: IF_OPER_STATUS = IF_OPER_STATUS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusDormant: IF_OPER_STATUS = IF_OPER_STATUS(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusNotPresent: IF_OPER_STATUS = IF_OPER_STATUS(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusLowerLayerDown: IF_OPER_STATUS = IF_OPER_STATUS(7i32);
impl ::core::marker::Copy for IF_OPER_STATUS {}
impl ::core::clone::Clone for IF_OPER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IF_OPER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IF_OPER_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IF_OPER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IF_OPER_STATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IF_PHYSICAL_ADDRESS_LH {
    pub Length: u16,
    pub Address: [u8; 32],
}
impl ::core::marker::Copy for IF_PHYSICAL_ADDRESS_LH {}
impl ::core::clone::Clone for IF_PHYSICAL_ADDRESS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IF_PHYSICAL_ADDRESS_LH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IF_PHYSICAL_ADDRESS_LH").field("Length", &self.Length).field("Address", &self.Address).finish()
    }
}
unsafe impl ::windows::core::Abi for IF_PHYSICAL_ADDRESS_LH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IF_PHYSICAL_ADDRESS_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IF_PHYSICAL_ADDRESS_LH>()) == 0 }
    }
}
impl ::core::cmp::Eq for IF_PHYSICAL_ADDRESS_LH {}
impl ::core::default::Default for IF_PHYSICAL_ADDRESS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ROW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_STATUS: u32 = 25u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_A12MPPSWITCH: u32 = 130u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_AAL2: u32 = 187u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_AAL5: u32 = 49u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ADSL: u32 = 94u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_AFLANE_8023: u32 = 59u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_AFLANE_8025: u32 = 60u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ARAP: u32 = 88u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ARCNET: u32 = 35u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ARCNET_PLUS: u32 = 36u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ASYNC: u32 = 84u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM: u32 = 37u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_DXI: u32 = 105u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_FUNI: u32 = 106u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_IMA: u32 = 107u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_LOGICAL: u32 = 80u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_RADIO: u32 = 189u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_SUBINTERFACE: u32 = 134u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_VCI_ENDPT: u32 = 194u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_VIRTUAL: u32 = 149u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_BASIC_ISDN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_BGP_POLICY_ACCOUNTING: u32 = 162u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_BSC: u32 = 83u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_CCTEMUL: u32 = 61u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_CES: u32 = 133u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_CHANNEL: u32 = 70u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_CNR: u32 = 85u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_COFFEE: u32 = 132u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_COMPOSITELINK: u32 = 155u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DCN: u32 = 141u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DDN_X25: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DIGITALPOWERLINE: u32 = 138u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DIGITAL_WRAPPER_OVERHEAD_CHANNEL: u32 = 186u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DLSW: u32 = 74u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DOCSCABLE_DOWNSTREAM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DOCSCABLE_MACLAYER: u32 = 127u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DOCSCABLE_UPSTREAM: u32 = 129u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DS0: u32 = 81u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DS0_BUNDLE: u32 = 82u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DS1: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DS1_FDL: u32 = 170u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DS3: u32 = 30u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DTM: u32 = 140u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DVBRCC_DOWNSTREAM: u32 = 147u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DVBRCC_MACLAYER: u32 = 146u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DVBRCC_UPSTREAM: u32 = 148u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DVB_ASI_IN: u32 = 172u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DVB_ASI_OUT: u32 = 173u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_E1: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_EON: u32 = 25u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_EPLRS: u32 = 87u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ESCON: u32 = 73u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ETHERNET_3MBIT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ETHERNET_CSMACD: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FAST: u32 = 125u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FASTETHER: u32 = 62u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FASTETHER_FX: u32 = 69u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FDDI: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FIBRECHANNEL: u32 = 56u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FRAMERELAY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FRAMERELAY_INTERCONNECT: u32 = 58u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FRAMERELAY_MPI: u32 = 92u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FRAMERELAY_SERVICE: u32 = 44u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FRF16_MFR_BUNDLE: u32 = 163u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FR_DLCI_ENDPT: u32 = 193u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FR_FORWARD: u32 = 158u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_G703_2MB: u32 = 67u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_G703_64K: u32 = 66u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_GIGABITETHERNET: u32 = 117u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_GR303_IDT: u32 = 178u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_GR303_RDT: u32 = 177u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_H323_GATEKEEPER: u32 = 164u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_H323_PROXY: u32 = 165u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HDH_1822: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HDLC: u32 = 118u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HDSL2: u32 = 168u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HIPERLAN2: u32 = 183u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HIPPI: u32 = 47u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HIPPIINTERFACE: u32 = 57u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HOSTPAD: u32 = 90u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HSSI: u32 = 46u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HYPERCHANNEL: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IBM370PARCHAN: u32 = 72u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IDSL: u32 = 154u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IEEE1394: u32 = 144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IEEE80211: u32 = 71u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IEEE80212: u32 = 55u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IEEE802154: u32 = 259u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IEEE80216_WMAN: u32 = 237u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IEEE8023AD_LAG: u32 = 161u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IF_GSN: u32 = 145u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IMT: u32 = 190u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_INTERLEAVE: u32 = 124u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IP: u32 = 126u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IPFORWARD: u32 = 142u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IPOVER_ATM: u32 = 114u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IPOVER_CDLC: u32 = 109u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IPOVER_CLAW: u32 = 110u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IPSWITCH: u32 = 78u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IS088023_CSMACD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISDN: u32 = 63u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISDN_S: u32 = 75u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISDN_U: u32 = 76u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88022_LLC: u32 = 41u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88024_TOKENBUS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88025R_DTR: u32 = 86u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88025_CRFPRINT: u32 = 98u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88025_FIBER: u32 = 115u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88025_TOKENRING: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88026_MAN: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISUP: u32 = 179u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_L2_VLAN: u32 = 135u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_L3_IPVLAN: u32 = 136u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_L3_IPXVLAN: u32 = 137u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_LAP_B: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_LAP_D: u32 = 77u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_LAP_F: u32 = 119u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_LOCALTALK: u32 = 42u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MEDIAMAILOVERIP: u32 = 139u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MF_SIGLINK: u32 = 167u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MIO_X25: u32 = 38u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MODEM: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MPC: u32 = 113u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MPLS: u32 = 166u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MPLS_TUNNEL: u32 = 150u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MSDSL: u32 = 143u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MVL: u32 = 191u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MYRINET: u32 = 99u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_NFAS: u32 = 175u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_NSIP: u32 = 27u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_OPTICAL_CHANNEL: u32 = 195u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_OPTICAL_TRANSPORT: u32 = 196u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PARA: u32 = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PLC: u32 = 174u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_POS: u32 = 171u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PPP: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PPPMULTILINKBUNDLE: u32 = 108u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PRIMARY_ISDN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_BWA_P2MP: u32 = 184u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_CNLS: u32 = 89u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_DOCS_WIRELESS_DOWNSTREAM: u32 = 181u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_DOCS_WIRELESS_MACLAYER: u32 = 180u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_DOCS_WIRELESS_UPSTREAM: u32 = 182u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_MULTIPLEXOR: u32 = 54u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_POINT2POINT_SERIAL: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_VIRTUAL: u32 = 53u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_WIRELESS_P2P: u32 = 157u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROTEON_10MBIT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROTEON_80MBIT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_QLLC: u32 = 68u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_RADIO_MAC: u32 = 188u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_RADSL: u32 = 95u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_REACH_DSL: u32 = 192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_REGULAR_1822: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_RFC1483: u32 = 159u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_RFC877_X25: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_RS232: u32 = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_RSRB: u32 = 79u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SDLC: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SDSL: u32 = 96u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SHDSL: u32 = 169u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SIP: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SLIP: u32 = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SMDS_DXI: u32 = 43u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SMDS_ICIP: u32 = 52u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SOFTWARE_LOOPBACK: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SONET: u32 = 39u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SONET_OVERHEAD_CHANNEL: u32 = 185u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SONET_PATH: u32 = 50u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SONET_VT: u32 = 51u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SRP: u32 = 151u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SS7_SIGLINK: u32 = 156u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_STACKTOSTACK: u32 = 111u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_STARLAN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_TDLC: u32 = 116u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_TERMPAD: u32 = 91u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_TR008: u32 = 176u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_TRANSPHDLC: u32 = 123u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_TUNNEL: u32 = 131u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ULTRA: u32 = 29u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_USB: u32 = 160u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_V11: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_V35: u32 = 45u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_V36: u32 = 65u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_V37: u32 = 120u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VDSL: u32 = 97u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VIRTUALIPADDRESS: u32 = 112u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICEOVERATM: u32 = 152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICEOVERFRAMERELAY: u32 = 153u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICE_EM: u32 = 100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICE_ENCAP: u32 = 103u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICE_FXO: u32 = 101u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICE_FXS: u32 = 102u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICE_OVERIP: u32 = 104u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_WWANPP: u32 = 243u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_WWANPP2: u32 = 244u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_X213: u32 = 93u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_X25_HUNTGROUP: u32 = 122u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_X25_MLP: u32 = 121u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_X25_PLE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_XBOX_WIRELESS: u32 = 281u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct INTERFACE_HARDWARE_CROSSTIMESTAMP {
    pub SystemTimestamp1: u64,
    pub HardwareClockTimestamp: u64,
    pub SystemTimestamp2: u64,
}
impl ::core::marker::Copy for INTERFACE_HARDWARE_CROSSTIMESTAMP {}
impl ::core::clone::Clone for INTERFACE_HARDWARE_CROSSTIMESTAMP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERFACE_HARDWARE_CROSSTIMESTAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACE_HARDWARE_CROSSTIMESTAMP").field("SystemTimestamp1", &self.SystemTimestamp1).field("HardwareClockTimestamp", &self.HardwareClockTimestamp).field("SystemTimestamp2", &self.SystemTimestamp2).finish()
    }
}
unsafe impl ::windows::core::Abi for INTERFACE_HARDWARE_CROSSTIMESTAMP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INTERFACE_HARDWARE_CROSSTIMESTAMP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERFACE_HARDWARE_CROSSTIMESTAMP>()) == 0 }
    }
}
impl ::core::cmp::Eq for INTERFACE_HARDWARE_CROSSTIMESTAMP {}
impl ::core::default::Default for INTERFACE_HARDWARE_CROSSTIMESTAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    pub PtpV2OverUdpIPv4EventMessageReceive: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv4AllMessageReceive: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv4EventMessageTransmit: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv4AllMessageTransmit: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6EventMessageReceive: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6AllMessageReceive: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6EventMessageTransmit: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6AllMessageTransmit: super::super::Foundation::BOOLEAN,
    pub AllReceive: super::super::Foundation::BOOLEAN,
    pub AllTransmit: super::super::Foundation::BOOLEAN,
    pub TaggedTransmit: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES")
            .field("PtpV2OverUdpIPv4EventMessageReceive", &self.PtpV2OverUdpIPv4EventMessageReceive)
            .field("PtpV2OverUdpIPv4AllMessageReceive", &self.PtpV2OverUdpIPv4AllMessageReceive)
            .field("PtpV2OverUdpIPv4EventMessageTransmit", &self.PtpV2OverUdpIPv4EventMessageTransmit)
            .field("PtpV2OverUdpIPv4AllMessageTransmit", &self.PtpV2OverUdpIPv4AllMessageTransmit)
            .field("PtpV2OverUdpIPv6EventMessageReceive", &self.PtpV2OverUdpIPv6EventMessageReceive)
            .field("PtpV2OverUdpIPv6AllMessageReceive", &self.PtpV2OverUdpIPv6AllMessageReceive)
            .field("PtpV2OverUdpIPv6EventMessageTransmit", &self.PtpV2OverUdpIPv6EventMessageTransmit)
            .field("PtpV2OverUdpIPv6AllMessageTransmit", &self.PtpV2OverUdpIPv6AllMessageTransmit)
            .field("AllReceive", &self.AllReceive)
            .field("AllTransmit", &self.AllTransmit)
            .field("TaggedTransmit", &self.TaggedTransmit)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    pub AllReceive: super::super::Foundation::BOOLEAN,
    pub AllTransmit: super::super::Foundation::BOOLEAN,
    pub TaggedTransmit: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES").field("AllReceive", &self.AllReceive).field("AllTransmit", &self.AllTransmit).field("TaggedTransmit", &self.TaggedTransmit).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERFACE_TIMESTAMP_CAPABILITIES {
    pub HardwareClockFrequencyHz: u64,
    pub SupportsCrossTimestamp: super::super::Foundation::BOOLEAN,
    pub HardwareCapabilities: INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES,
    pub SoftwareCapabilities: INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERFACE_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERFACE_TIMESTAMP_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERFACE_TIMESTAMP_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACE_TIMESTAMP_CAPABILITIES").field("HardwareClockFrequencyHz", &self.HardwareClockFrequencyHz).field("SupportsCrossTimestamp", &self.SupportsCrossTimestamp).field("HardwareCapabilities", &self.HardwareCapabilities).field("SoftwareCapabilities", &self.SoftwareCapabilities).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERFACE_TIMESTAMP_CAPABILITIES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERFACE_TIMESTAMP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERFACE_TIMESTAMP_CAPABILITIES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERFACE_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERFACE_TIMESTAMP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INTERNAL_IF_OPER_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_OPER_STATUS_NON_OPERATIONAL: INTERNAL_IF_OPER_STATUS = INTERNAL_IF_OPER_STATUS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_OPER_STATUS_UNREACHABLE: INTERNAL_IF_OPER_STATUS = INTERNAL_IF_OPER_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_OPER_STATUS_DISCONNECTED: INTERNAL_IF_OPER_STATUS = INTERNAL_IF_OPER_STATUS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_OPER_STATUS_CONNECTING: INTERNAL_IF_OPER_STATUS = INTERNAL_IF_OPER_STATUS(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_OPER_STATUS_CONNECTED: INTERNAL_IF_OPER_STATUS = INTERNAL_IF_OPER_STATUS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_OPER_STATUS_OPERATIONAL: INTERNAL_IF_OPER_STATUS = INTERNAL_IF_OPER_STATUS(5i32);
impl ::core::marker::Copy for INTERNAL_IF_OPER_STATUS {}
impl ::core::clone::Clone for INTERNAL_IF_OPER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INTERNAL_IF_OPER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INTERNAL_IF_OPER_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for INTERNAL_IF_OPER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNAL_IF_OPER_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IOCTL_ARP_SEND_REQUEST: u32 = 103u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IOCTL_IP_ADDCHANGE_NOTIFY_REQUEST: u32 = 102u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IOCTL_IP_GET_BEST_INTERFACE: u32 = 105u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IOCTL_IP_INTERFACE_INFO: u32 = 104u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IOCTL_IP_RTCHANGE_NOTIFY_REQUEST: u32 = 101u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IOCTL_IP_UNIDIRECTIONAL_ADAPTER_ADDRESS: u32 = 106u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP6_STATS: u32 = 36u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IPRTRMGR_PID: u32 = 10000u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IPV6_ADDRESS_EX {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl ::core::marker::Copy for IPV6_ADDRESS_EX {}
impl ::core::clone::Clone for IPV6_ADDRESS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPV6_ADDRESS_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_ADDRESS_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_ADDRESS_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_ADDRESS_EX {}
impl ::core::default::Default for IPV6_ADDRESS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IPV6_GLOBAL_INFO: u32 = 4294901775u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IPV6_ROUTE_INFO: u32 = 4294901776u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ADDRESSES_LH {
    pub Anonymous1: IP_ADAPTER_ADDRESSES_LH_0,
    pub Next: *mut IP_ADAPTER_ADDRESSES_LH,
    pub AdapterName: ::windows::core::PSTR,
    pub FirstUnicastAddress: *mut IP_ADAPTER_UNICAST_ADDRESS_LH,
    pub FirstAnycastAddress: *mut IP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub FirstMulticastAddress: *mut IP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub FirstDnsServerAddress: *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub DnsSuffix: ::windows::core::PWSTR,
    pub Description: ::windows::core::PWSTR,
    pub FriendlyName: ::windows::core::PWSTR,
    pub PhysicalAddress: [u8; 8],
    pub PhysicalAddressLength: u32,
    pub Anonymous2: IP_ADAPTER_ADDRESSES_LH_1,
    pub Mtu: u32,
    pub IfType: u32,
    pub OperStatus: IF_OPER_STATUS,
    pub Ipv6IfIndex: u32,
    pub ZoneIndices: [u32; 16],
    pub FirstPrefix: *mut IP_ADAPTER_PREFIX_XP,
    pub TransmitLinkSpeed: u64,
    pub ReceiveLinkSpeed: u64,
    pub FirstWinsServerAddress: *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH,
    pub FirstGatewayAddress: *mut IP_ADAPTER_GATEWAY_ADDRESS_LH,
    pub Ipv4Metric: u32,
    pub Ipv6Metric: u32,
    pub Luid: NET_LUID_LH,
    pub Dhcpv4Server: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub CompartmentId: u32,
    pub NetworkGuid: ::windows::core::GUID,
    pub ConnectionType: NET_IF_CONNECTION_TYPE,
    pub TunnelType: TUNNEL_TYPE,
    pub Dhcpv6Server: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub Dhcpv6ClientDuid: [u8; 130],
    pub Dhcpv6ClientDuidLength: u32,
    pub Dhcpv6Iaid: u32,
    pub FirstDnsSuffix: *mut IP_ADAPTER_DNS_SUFFIX,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_LH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_ADDRESSES_LH {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_ADDRESSES_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_ADDRESSES_LH>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_ADDRESSES_LH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ADDRESSES_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_ADDRESSES_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_LH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_ADDRESSES_LH_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_ADDRESSES_LH_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_ADDRESSES_LH_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_ADDRESSES_LH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ADDRESSES_LH_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ADDRESSES_LH_0_0 {
    pub Length: u32,
    pub IfIndex: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_LH_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_LH_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for IP_ADAPTER_ADDRESSES_LH_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_ADDRESSES_LH_0_0").field("Length", &self.Length).field("IfIndex", &self.IfIndex).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_ADDRESSES_LH_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_ADDRESSES_LH_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_ADDRESSES_LH_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_ADDRESSES_LH_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ADDRESSES_LH_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_ADDRESSES_LH_1 {
    pub Flags: u32,
    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_LH_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_LH_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_ADDRESSES_LH_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_ADDRESSES_LH_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_ADDRESSES_LH_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_ADDRESSES_LH_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ADDRESSES_LH_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ADDRESSES_LH_1_0 {
    pub _bitfield: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_LH_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_LH_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for IP_ADAPTER_ADDRESSES_LH_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_ADDRESSES_LH_1_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_ADDRESSES_LH_1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_ADDRESSES_LH_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_ADDRESSES_LH_1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_ADDRESSES_LH_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ADDRESSES_LH_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ADDRESSES_XP {
    pub Anonymous: IP_ADAPTER_ADDRESSES_XP_0,
    pub Next: *mut IP_ADAPTER_ADDRESSES_XP,
    pub AdapterName: ::windows::core::PSTR,
    pub FirstUnicastAddress: *mut IP_ADAPTER_UNICAST_ADDRESS_XP,
    pub FirstAnycastAddress: *mut IP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub FirstMulticastAddress: *mut IP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub FirstDnsServerAddress: *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub DnsSuffix: ::windows::core::PWSTR,
    pub Description: ::windows::core::PWSTR,
    pub FriendlyName: ::windows::core::PWSTR,
    pub PhysicalAddress: [u8; 8],
    pub PhysicalAddressLength: u32,
    pub Flags: u32,
    pub Mtu: u32,
    pub IfType: u32,
    pub OperStatus: IF_OPER_STATUS,
    pub Ipv6IfIndex: u32,
    pub ZoneIndices: [u32; 16],
    pub FirstPrefix: *mut IP_ADAPTER_PREFIX_XP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_ADDRESSES_XP {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_ADDRESSES_XP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_ADDRESSES_XP>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_ADDRESSES_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ADDRESSES_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_ADDRESSES_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_ADDRESSES_XP_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_XP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_ADDRESSES_XP_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_ADDRESSES_XP_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_ADDRESSES_XP_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_ADDRESSES_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ADDRESSES_XP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ADDRESSES_XP_0_0 {
    pub Length: u32,
    pub IfIndex: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_XP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for IP_ADAPTER_ADDRESSES_XP_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_ADDRESSES_XP_0_0").field("Length", &self.Length).field("IfIndex", &self.IfIndex).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_ADDRESSES_XP_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_ADDRESSES_XP_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_ADDRESSES_XP_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_ADDRESSES_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ADDRESSES_XP_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_ADDRESS_DNS_ELIGIBLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_ADDRESS_TRANSIENT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_ANYCAST_ADDRESS_XP_0,
    pub Next: *mut IP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ANYCAST_ADDRESS_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ANYCAST_ADDRESS_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_ANYCAST_ADDRESS_XP {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_ANYCAST_ADDRESS_XP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_ANYCAST_ADDRESS_XP>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_ANYCAST_ADDRESS_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ANYCAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_ANYCAST_ADDRESS_XP_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0").field("Length", &self.Length).field("Flags", &self.Flags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_DDNS_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_DHCP_ENABLED: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0,
    pub Next: *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_DNS_SERVER_ADDRESS_XP>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Reserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0").field("Length", &self.Length).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IP_ADAPTER_DNS_SUFFIX {
    pub Next: *mut IP_ADAPTER_DNS_SUFFIX,
    pub String: [u16; 256],
}
impl ::core::marker::Copy for IP_ADAPTER_DNS_SUFFIX {}
impl ::core::clone::Clone for IP_ADAPTER_DNS_SUFFIX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IP_ADAPTER_DNS_SUFFIX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_DNS_SUFFIX").field("Next", &self.Next).field("String", &self.String).finish()
    }
}
unsafe impl ::windows::core::Abi for IP_ADAPTER_DNS_SUFFIX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IP_ADAPTER_DNS_SUFFIX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_DNS_SUFFIX>()) == 0 }
    }
}
impl ::core::cmp::Eq for IP_ADAPTER_DNS_SUFFIX {}
impl ::core::default::Default for IP_ADAPTER_DNS_SUFFIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_GATEWAY_ADDRESS_LH_0,
    pub Next: *mut IP_ADAPTER_GATEWAY_ADDRESS_LH,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_GATEWAY_ADDRESS_LH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_GATEWAY_ADDRESS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_GATEWAY_ADDRESS_LH {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_GATEWAY_ADDRESS_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_GATEWAY_ADDRESS_LH>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_GATEWAY_ADDRESS_LH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_GATEWAY_ADDRESS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_GATEWAY_ADDRESS_LH_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
    pub Length: u32,
    pub Reserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0").field("Length", &self.Length).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IP_ADAPTER_INDEX_MAP {
    pub Index: u32,
    pub Name: [u16; 128],
}
impl ::core::marker::Copy for IP_ADAPTER_INDEX_MAP {}
impl ::core::clone::Clone for IP_ADAPTER_INDEX_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IP_ADAPTER_INDEX_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_INDEX_MAP").field("Index", &self.Index).field("Name", &self.Name).finish()
    }
}
unsafe impl ::windows::core::Abi for IP_ADAPTER_INDEX_MAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IP_ADAPTER_INDEX_MAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_INDEX_MAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for IP_ADAPTER_INDEX_MAP {}
impl ::core::default::Default for IP_ADAPTER_INDEX_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IP_ADAPTER_INFO {
    pub Next: *mut IP_ADAPTER_INFO,
    pub ComboIndex: u32,
    pub AdapterName: [super::super::Foundation::CHAR; 260],
    pub Description: [super::super::Foundation::CHAR; 132],
    pub AddressLength: u32,
    pub Address: [u8; 8],
    pub Index: u32,
    pub Type: u32,
    pub DhcpEnabled: u32,
    pub CurrentIpAddress: *mut IP_ADDR_STRING,
    pub IpAddressList: IP_ADDR_STRING,
    pub GatewayList: IP_ADDR_STRING,
    pub DhcpServer: IP_ADDR_STRING,
    pub HaveWins: super::super::Foundation::BOOL,
    pub PrimaryWinsServer: IP_ADDR_STRING,
    pub SecondaryWinsServer: IP_ADDR_STRING,
    pub LeaseObtained: i64,
    pub LeaseExpires: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IP_ADAPTER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IP_ADAPTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IP_ADAPTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_INFO")
            .field("Next", &self.Next)
            .field("ComboIndex", &self.ComboIndex)
            .field("AdapterName", &self.AdapterName)
            .field("Description", &self.Description)
            .field("AddressLength", &self.AddressLength)
            .field("Address", &self.Address)
            .field("Index", &self.Index)
            .field("Type", &self.Type)
            .field("DhcpEnabled", &self.DhcpEnabled)
            .field("CurrentIpAddress", &self.CurrentIpAddress)
            .field("IpAddressList", &self.IpAddressList)
            .field("GatewayList", &self.GatewayList)
            .field("DhcpServer", &self.DhcpServer)
            .field("HaveWins", &self.HaveWins)
            .field("PrimaryWinsServer", &self.PrimaryWinsServer)
            .field("SecondaryWinsServer", &self.SecondaryWinsServer)
            .field("LeaseObtained", &self.LeaseObtained)
            .field("LeaseExpires", &self.LeaseExpires)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IP_ADAPTER_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IP_ADAPTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IP_ADAPTER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IP_ADAPTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_IPV4_ENABLED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_IPV6_ENABLED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_IPV6_MANAGE_ADDRESS_CONFIG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_IPV6_OTHER_STATEFUL_CONFIG: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_MULTICAST_ADDRESS_XP_0,
    pub Next: *mut IP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_MULTICAST_ADDRESS_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_MULTICAST_ADDRESS_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_MULTICAST_ADDRESS_XP {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_MULTICAST_ADDRESS_XP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_MULTICAST_ADDRESS_XP>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_MULTICAST_ADDRESS_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_MULTICAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_MULTICAST_ADDRESS_XP_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0").field("Length", &self.Length).field("Flags", &self.Flags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_NETBIOS_OVER_TCPIP_ENABLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_NO_MULTICAST: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IP_ADAPTER_ORDER_MAP {
    pub NumAdapters: u32,
    pub AdapterOrder: [u32; 1],
}
impl ::core::marker::Copy for IP_ADAPTER_ORDER_MAP {}
impl ::core::clone::Clone for IP_ADAPTER_ORDER_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IP_ADAPTER_ORDER_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_ORDER_MAP").field("NumAdapters", &self.NumAdapters).field("AdapterOrder", &self.AdapterOrder).finish()
    }
}
unsafe impl ::windows::core::Abi for IP_ADAPTER_ORDER_MAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IP_ADAPTER_ORDER_MAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_ORDER_MAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for IP_ADAPTER_ORDER_MAP {}
impl ::core::default::Default for IP_ADAPTER_ORDER_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_PREFIX_XP {
    pub Anonymous: IP_ADAPTER_PREFIX_XP_0,
    pub Next: *mut IP_ADAPTER_PREFIX_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub PrefixLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_PREFIX_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_PREFIX_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_PREFIX_XP {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_PREFIX_XP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_PREFIX_XP>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_PREFIX_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_PREFIX_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_PREFIX_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_PREFIX_XP_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_PREFIX_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_PREFIX_XP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_PREFIX_XP_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_PREFIX_XP_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_PREFIX_XP_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_PREFIX_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_PREFIX_XP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_PREFIX_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_PREFIX_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_PREFIX_XP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for IP_ADAPTER_PREFIX_XP_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_PREFIX_XP_0_0").field("Length", &self.Length).field("Flags", &self.Flags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_PREFIX_XP_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_PREFIX_XP_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_PREFIX_XP_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_PREFIX_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_PREFIX_XP_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_RECEIVE_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_REGISTER_ADAPTER_SUFFIX: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_UNICAST_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_LH_0,
    pub Next: *mut IP_ADAPTER_UNICAST_ADDRESS_LH,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub PrefixOrigin: super::super::Networking::WinSock::NL_PREFIX_ORIGIN,
    pub SuffixOrigin: super::super::Networking::WinSock::NL_SUFFIX_ORIGIN,
    pub DadState: super::super::Networking::WinSock::NL_DAD_STATE,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub LeaseLifetime: u32,
    pub OnLinkPrefixLength: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_UNICAST_ADDRESS_LH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_UNICAST_ADDRESS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_UNICAST_ADDRESS_LH {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_UNICAST_ADDRESS_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_UNICAST_ADDRESS_LH>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_UNICAST_ADDRESS_LH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_UNICAST_ADDRESS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_LH_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_UNICAST_ADDRESS_LH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_UNICAST_ADDRESS_LH_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_UNICAST_ADDRESS_LH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_UNICAST_ADDRESS_LH_0_0").field("Length", &self.Length).field("Flags", &self.Flags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_UNICAST_ADDRESS_LH_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_UNICAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_XP_0,
    pub Next: *mut IP_ADAPTER_UNICAST_ADDRESS_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub PrefixOrigin: super::super::Networking::WinSock::NL_PREFIX_ORIGIN,
    pub SuffixOrigin: super::super::Networking::WinSock::NL_SUFFIX_ORIGIN,
    pub DadState: super::super::Networking::WinSock::NL_DAD_STATE,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub LeaseLifetime: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_UNICAST_ADDRESS_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_UNICAST_ADDRESS_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_UNICAST_ADDRESS_XP {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_UNICAST_ADDRESS_XP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_UNICAST_ADDRESS_XP>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_UNICAST_ADDRESS_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_UNICAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_UNICAST_ADDRESS_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_UNICAST_ADDRESS_XP_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_UNICAST_ADDRESS_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_UNICAST_ADDRESS_XP_0_0").field("Length", &self.Length).field("Flags", &self.Flags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_UNICAST_ADDRESS_XP_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0,
    pub Next: *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_WINS_SERVER_ADDRESS_LH>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
    pub Length: u32,
    pub Reserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0").field("Length", &self.Length).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADDRESS_PREFIX {
    pub Prefix: super::super::Networking::WinSock::SOCKADDR_INET,
    pub PrefixLength: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADDRESS_PREFIX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADDRESS_PREFIX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for IP_ADDRESS_PREFIX {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for IP_ADDRESS_PREFIX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADDRESS_PREFIX>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for IP_ADDRESS_PREFIX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADDRESS_PREFIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IP_ADDRESS_STRING {
    pub String: [super::super::Foundation::CHAR; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IP_ADDRESS_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IP_ADDRESS_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IP_ADDRESS_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADDRESS_STRING").field("String", &self.String).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IP_ADDRESS_STRING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IP_ADDRESS_STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADDRESS_STRING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IP_ADDRESS_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IP_ADDRESS_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADDRROW: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADDRTABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADDR_ADDED: u32 = 11023u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADDR_DELETED: u32 = 11019u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IP_ADDR_STRING {
    pub Next: *mut IP_ADDR_STRING,
    pub IpAddress: IP_ADDRESS_STRING,
    pub IpMask: IP_ADDRESS_STRING,
    pub Context: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IP_ADDR_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IP_ADDR_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IP_ADDR_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADDR_STRING").field("Next", &self.Next).field("IpAddress", &self.IpAddress).field("IpMask", &self.IpMask).field("Context", &self.Context).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IP_ADDR_STRING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IP_ADDR_STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_ADDR_STRING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IP_ADDR_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IP_ADDR_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BAD_DESTINATION: u32 = 11018u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BAD_HEADER: u32 = 11042u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BAD_OPTION: u32 = 11007u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BAD_REQ: u32 = 11011u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BAD_ROUTE: u32 = 11012u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BIND_ADAPTER: u32 = 11026u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BUF_TOO_SMALL: u32 = 11001u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEMAND_DIAL_FILTER_INFO: u32 = 4294901769u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEMAND_DIAL_FILTER_INFO_V6: u32 = 4294901779u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_ADDR_UNREACHABLE: u32 = 11003u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_HOST_UNREACHABLE: u32 = 11003u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_NET_UNREACHABLE: u32 = 11002u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_NO_ROUTE: u32 = 11002u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_PORT_UNREACHABLE: u32 = 11005u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_PROHIBITED: u32 = 11004u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_PROT_UNREACHABLE: u32 = 11004u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_SCOPE_MISMATCH: u32 = 11045u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_UNREACHABLE: u32 = 11040u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEVICE_DOES_NOT_EXIST: u32 = 11028u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DUPLICATE_ADDRESS: u32 = 11029u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DUPLICATE_IPADD: u32 = 11034u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_EXPORT_INCLUDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FILTER_ENABLE_INFO: u32 = 4294901781u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FILTER_ENABLE_INFO_V6: u32 = 4294901782u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FLAG_DF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FLAG_REVERSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FORWARDNUMBER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FORWARDROW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FORWARDTABLE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_GENERAL_FAILURE: u32 = 11050u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_GENERAL_INFO_BASE: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_GLOBAL_INFO: u32 = 4294901763u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_HOP_LIMIT_EXCEEDED: u32 = 11013u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_HW_ERROR: u32 = 11008u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ICMP_ERROR: u32 = 11044u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_IFFILTER_INFO: u32 = 4294901773u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_IFFILTER_INFO_V6: u32 = 4294901780u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IP_INTERFACE_INFO {
    pub NumAdapters: i32,
    pub Adapter: [IP_ADAPTER_INDEX_MAP; 1],
}
impl ::core::marker::Copy for IP_INTERFACE_INFO {}
impl ::core::clone::Clone for IP_INTERFACE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IP_INTERFACE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_INTERFACE_INFO").field("NumAdapters", &self.NumAdapters).field("Adapter", &self.Adapter).finish()
    }
}
unsafe impl ::windows::core::Abi for IP_INTERFACE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IP_INTERFACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_INTERFACE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for IP_INTERFACE_INFO {}
impl ::core::default::Default for IP_INTERFACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_INTERFACE_METRIC_CHANGE: u32 = 11030u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_INTERFACE_STATUS_INFO: u32 = 4294901764u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_INTERFACE_WOL_CAPABILITY_CHANGE: u32 = 11033u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_IN_FILTER_INFO: u32 = 4294901761u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_IN_FILTER_INFO_V6: u32 = 4294901777u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_IPINIP_CFG_INFO: u32 = 4294901772u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_MCAST_BOUNDARY_INFO: u32 = 4294901771u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IP_MCAST_COUNTER_INFO {
    pub InMcastOctets: u64,
    pub OutMcastOctets: u64,
    pub InMcastPkts: u64,
    pub OutMcastPkts: u64,
}
impl ::core::marker::Copy for IP_MCAST_COUNTER_INFO {}
impl ::core::clone::Clone for IP_MCAST_COUNTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IP_MCAST_COUNTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_MCAST_COUNTER_INFO").field("InMcastOctets", &self.InMcastOctets).field("OutMcastOctets", &self.OutMcastOctets).field("InMcastPkts", &self.InMcastPkts).field("OutMcastPkts", &self.OutMcastPkts).finish()
    }
}
unsafe impl ::windows::core::Abi for IP_MCAST_COUNTER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IP_MCAST_COUNTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_MCAST_COUNTER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for IP_MCAST_COUNTER_INFO {}
impl ::core::default::Default for IP_MCAST_COUNTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_MCAST_HEARBEAT_INFO: u32 = 4294901770u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_MCAST_LIMIT_INFO: u32 = 4294901774u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_MEDIA_CONNECT: u32 = 11024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_MEDIA_DISCONNECT: u32 = 11025u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_MTU_CHANGE: u32 = 11021u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_NEGOTIATING_IPSEC: u32 = 11032u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_NETROW: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_NETTABLE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_NO_RESOURCES: u32 = 11006u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_OPTION_TOO_BIG: u32 = 11017u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_OUT_FILTER_INFO: u32 = 4294901762u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_OUT_FILTER_INFO_V6: u32 = 4294901778u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_PACKET_TOO_BIG: u32 = 11009u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_PARAMETER_PROBLEM: u32 = 11015u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_PARAM_PROBLEM: u32 = 11015u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_PENDING: u32 = 11255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IP_PER_ADAPTER_INFO_W2KSP1 {
    pub AutoconfigEnabled: u32,
    pub AutoconfigActive: u32,
    pub CurrentDnsServer: *mut IP_ADDR_STRING,
    pub DnsServerList: IP_ADDR_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IP_PER_ADAPTER_INFO_W2KSP1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IP_PER_ADAPTER_INFO_W2KSP1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IP_PER_ADAPTER_INFO_W2KSP1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_PER_ADAPTER_INFO_W2KSP1").field("AutoconfigEnabled", &self.AutoconfigEnabled).field("AutoconfigActive", &self.AutoconfigActive).field("CurrentDnsServer", &self.CurrentDnsServer).field("DnsServerList", &self.DnsServerList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IP_PER_ADAPTER_INFO_W2KSP1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IP_PER_ADAPTER_INFO_W2KSP1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_PER_ADAPTER_INFO_W2KSP1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IP_PER_ADAPTER_INFO_W2KSP1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IP_PER_ADAPTER_INFO_W2KSP1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_PROT_PRIORITY_INFO: u32 = 4294901766u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_PROT_PRIORITY_INFO_EX: u32 = 4294901783u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_REASSEMBLY_TIME_EXCEEDED: u32 = 11014u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_RECONFIG_SECFLTR: u32 = 11031u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_REQ_TIMED_OUT: u32 = 11010u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ROUTER_DISC_INFO: u32 = 4294901767u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ROUTER_MANAGER_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ROUTE_INFO: u32 = 4294901765u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_SOURCE_QUENCH: u32 = 11016u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_SPEC_MTU_CHANGE: u32 = 11020u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_STATS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_STATUS_BASE: u32 = 11000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_TIME_EXCEEDED: u32 = 11041u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_TTL_EXPIRED_REASSEM: u32 = 11014u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_TTL_EXPIRED_TRANSIT: u32 = 11013u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_UNBIND_ADAPTER: u32 = 11027u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    pub NumAdapters: u32,
    pub Address: [u32; 1],
}
impl ::core::marker::Copy for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {}
impl ::core::clone::Clone for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_UNIDIRECTIONAL_ADAPTER_ADDRESS").field("NumAdapters", &self.NumAdapters).field("Address", &self.Address).finish()
    }
}
unsafe impl ::windows::core::Abi for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_UNIDIRECTIONAL_ADAPTER_ADDRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {}
impl ::core::default::Default for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_UNLOAD: u32 = 11022u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_UNRECOGNIZED_NEXT_HEADER: u32 = 11043u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn Icmp6CreateFile() -> IcmpHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Icmp6CreateFile() -> IcmpHandle;
        }
        ::core::mem::transmute(Icmp6CreateFile())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn Icmp6ParseReplies(replybuffer: *mut ::core::ffi::c_void, replysize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Icmp6ParseReplies(replybuffer: *mut ::core::ffi::c_void, replysize: u32) -> u32;
        }
        ::core::mem::transmute(Icmp6ParseReplies(::core::mem::transmute(replybuffer), ::core::mem::transmute(replysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn Icmp6SendEcho2<'a, Param0: ::windows::core::IntoParam<'a, IcmpHandle>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(icmphandle: Param0, event: Param1, apcroutine: super::super::System::WindowsProgramming::PIO_APC_ROUTINE, apccontext: *const ::core::ffi::c_void, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_IN6, destinationaddress: *const super::super::Networking::WinSock::SOCKADDR_IN6, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Icmp6SendEcho2(icmphandle: IcmpHandle, event: super::super::Foundation::HANDLE, apcroutine: ::windows::core::RawPtr, apccontext: *const ::core::ffi::c_void, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_IN6, destinationaddress: *const super::super::Networking::WinSock::SOCKADDR_IN6, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
        }
        ::core::mem::transmute(Icmp6SendEcho2(icmphandle.into_param().abi(), event.into_param().abi(), ::core::mem::transmute(apcroutine), ::core::mem::transmute(apccontext), ::core::mem::transmute(sourceaddress), ::core::mem::transmute(destinationaddress), ::core::mem::transmute(requestdata), ::core::mem::transmute(requestsize), ::core::mem::transmute(requestoptions), ::core::mem::transmute(replybuffer), ::core::mem::transmute(replysize), ::core::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IcmpCloseHandle<'a, Param0: ::windows::core::IntoParam<'a, IcmpHandle>>(icmphandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IcmpCloseHandle(icmphandle: IcmpHandle) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IcmpCloseHandle(icmphandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn IcmpCreateFile() -> IcmpHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IcmpCreateFile() -> IcmpHandle;
        }
        ::core::mem::transmute(IcmpCreateFile())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IcmpHandle(pub isize);
impl IcmpHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for IcmpHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for IcmpHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for IcmpHandle {}
impl ::core::fmt::Debug for IcmpHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IcmpHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for IcmpHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn IcmpParseReplies(replybuffer: *mut ::core::ffi::c_void, replysize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IcmpParseReplies(replybuffer: *mut ::core::ffi::c_void, replysize: u32) -> u32;
        }
        ::core::mem::transmute(IcmpParseReplies(::core::mem::transmute(replybuffer), ::core::mem::transmute(replysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn IcmpSendEcho<'a, Param0: ::windows::core::IntoParam<'a, IcmpHandle>>(icmphandle: Param0, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IcmpSendEcho(icmphandle: IcmpHandle, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
        }
        ::core::mem::transmute(IcmpSendEcho(icmphandle.into_param().abi(), ::core::mem::transmute(destinationaddress), ::core::mem::transmute(requestdata), ::core::mem::transmute(requestsize), ::core::mem::transmute(requestoptions), ::core::mem::transmute(replybuffer), ::core::mem::transmute(replysize), ::core::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn IcmpSendEcho2<'a, Param0: ::windows::core::IntoParam<'a, IcmpHandle>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(icmphandle: Param0, event: Param1, apcroutine: super::super::System::WindowsProgramming::PIO_APC_ROUTINE, apccontext: *const ::core::ffi::c_void, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IcmpSendEcho2(icmphandle: IcmpHandle, event: super::super::Foundation::HANDLE, apcroutine: ::windows::core::RawPtr, apccontext: *const ::core::ffi::c_void, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
        }
        ::core::mem::transmute(IcmpSendEcho2(icmphandle.into_param().abi(), event.into_param().abi(), ::core::mem::transmute(apcroutine), ::core::mem::transmute(apccontext), ::core::mem::transmute(destinationaddress), ::core::mem::transmute(requestdata), ::core::mem::transmute(requestsize), ::core::mem::transmute(requestoptions), ::core::mem::transmute(replybuffer), ::core::mem::transmute(replysize), ::core::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn IcmpSendEcho2Ex<'a, Param0: ::windows::core::IntoParam<'a, IcmpHandle>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(icmphandle: Param0, event: Param1, apcroutine: super::super::System::WindowsProgramming::PIO_APC_ROUTINE, apccontext: *const ::core::ffi::c_void, sourceaddress: u32, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IcmpSendEcho2Ex(icmphandle: IcmpHandle, event: super::super::Foundation::HANDLE, apcroutine: ::windows::core::RawPtr, apccontext: *const ::core::ffi::c_void, sourceaddress: u32, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
        }
        ::core::mem::transmute(IcmpSendEcho2Ex(icmphandle.into_param().abi(), event.into_param().abi(), ::core::mem::transmute(apcroutine), ::core::mem::transmute(apccontext), ::core::mem::transmute(sourceaddress), ::core::mem::transmute(destinationaddress), ::core::mem::transmute(requestdata), ::core::mem::transmute(requestsize), ::core::mem::transmute(requestoptions), ::core::mem::transmute(replybuffer), ::core::mem::transmute(replysize), ::core::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn InitializeIpForwardEntry(row: *mut MIB_IPFORWARD_ROW2) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeIpForwardEntry(row: *mut MIB_IPFORWARD_ROW2);
        }
        InitializeIpForwardEntry(::core::mem::transmute(row))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn InitializeIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW);
        }
        InitializeIpInterfaceEntry(::core::mem::transmute(row))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn InitializeUnicastIpAddressEntry(row: *mut MIB_UNICASTIPADDRESS_ROW) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeUnicastIpAddressEntry(row: *mut MIB_UNICASTIPADDRESS_ROW);
        }
        InitializeUnicastIpAddressEntry(::core::mem::transmute(row))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn IpReleaseAddress(adapterinfo: *const IP_ADAPTER_INDEX_MAP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IpReleaseAddress(adapterinfo: *const IP_ADAPTER_INDEX_MAP) -> u32;
        }
        ::core::mem::transmute(IpReleaseAddress(::core::mem::transmute(adapterinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn IpRenewAddress(adapterinfo: *const IP_ADAPTER_INDEX_MAP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IpRenewAddress(adapterinfo: *const IP_ADAPTER_INDEX_MAP) -> u32;
        }
        ::core::mem::transmute(IpRenewAddress(::core::mem::transmute(adapterinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const LB_DST_ADDR_USE_DSTADDR_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const LB_DST_ADDR_USE_SRCADDR_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const LB_DST_MASK_LATE_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const LB_SRC_ADDR_USE_DSTADDR_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const LB_SRC_ADDR_USE_SRCADDR_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const LB_SRC_MASK_LATE_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn LookupPersistentTcpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPersistentTcpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32;
        }
        ::core::mem::transmute(LookupPersistentTcpPortReservation(::core::mem::transmute(startport), ::core::mem::transmute(numberofports), ::core::mem::transmute(token)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn LookupPersistentUdpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPersistentUdpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32;
        }
        ::core::mem::transmute(LookupPersistentUdpPortReservation(::core::mem::transmute(startport), ::core::mem::transmute(numberofports), ::core::mem::transmute(token)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAXLEN_IFDESCR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAXLEN_PHYSADDR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_ADAPTER_ADDRESS_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_ADAPTER_DESCRIPTION_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_ADAPTER_NAME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_ADAPTER_NAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_DHCPV6_DUID_LENGTH: u32 = 130u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_DNS_SUFFIX_STRING_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_DOMAIN_NAME_LEN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_HOSTNAME_LEN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_IF_TYPE: u32 = 281u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_INTERFACE_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_IP_STATUS: u32 = 11050u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_MIB_OFFSET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_OPT_SIZE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_SCOPE_ID_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_SCOPE_NAME_LEN: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_BOUNDARY: u32 = 26u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_GLOBAL: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_IF_ENTRY: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_MFE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_MFE_STATS: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_MFE_STATS_EX: u32 = 35u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_SCOPE: u32 = 27u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIBICMPINFO {
    pub icmpInStats: MIBICMPSTATS,
    pub icmpOutStats: MIBICMPSTATS,
}
impl ::core::marker::Copy for MIBICMPINFO {}
impl ::core::clone::Clone for MIBICMPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIBICMPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIBICMPINFO").field("icmpInStats", &self.icmpInStats).field("icmpOutStats", &self.icmpOutStats).finish()
    }
}
unsafe impl ::windows::core::Abi for MIBICMPINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIBICMPINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIBICMPINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIBICMPINFO {}
impl ::core::default::Default for MIBICMPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIBICMPSTATS {
    pub dwMsgs: u32,
    pub dwErrors: u32,
    pub dwDestUnreachs: u32,
    pub dwTimeExcds: u32,
    pub dwParmProbs: u32,
    pub dwSrcQuenchs: u32,
    pub dwRedirects: u32,
    pub dwEchos: u32,
    pub dwEchoReps: u32,
    pub dwTimestamps: u32,
    pub dwTimestampReps: u32,
    pub dwAddrMasks: u32,
    pub dwAddrMaskReps: u32,
}
impl ::core::marker::Copy for MIBICMPSTATS {}
impl ::core::clone::Clone for MIBICMPSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIBICMPSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIBICMPSTATS")
            .field("dwMsgs", &self.dwMsgs)
            .field("dwErrors", &self.dwErrors)
            .field("dwDestUnreachs", &self.dwDestUnreachs)
            .field("dwTimeExcds", &self.dwTimeExcds)
            .field("dwParmProbs", &self.dwParmProbs)
            .field("dwSrcQuenchs", &self.dwSrcQuenchs)
            .field("dwRedirects", &self.dwRedirects)
            .field("dwEchos", &self.dwEchos)
            .field("dwEchoReps", &self.dwEchoReps)
            .field("dwTimestamps", &self.dwTimestamps)
            .field("dwTimestampReps", &self.dwTimestampReps)
            .field("dwAddrMasks", &self.dwAddrMasks)
            .field("dwAddrMaskReps", &self.dwAddrMaskReps)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MIBICMPSTATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIBICMPSTATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIBICMPSTATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIBICMPSTATS {}
impl ::core::default::Default for MIBICMPSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIBICMPSTATS_EX_XPSP1 {
    pub dwMsgs: u32,
    pub dwErrors: u32,
    pub rgdwTypeCount: [u32; 256],
}
impl ::core::marker::Copy for MIBICMPSTATS_EX_XPSP1 {}
impl ::core::clone::Clone for MIBICMPSTATS_EX_XPSP1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIBICMPSTATS_EX_XPSP1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIBICMPSTATS_EX_XPSP1").field("dwMsgs", &self.dwMsgs).field("dwErrors", &self.dwErrors).field("rgdwTypeCount", &self.rgdwTypeCount).finish()
    }
}
unsafe impl ::windows::core::Abi for MIBICMPSTATS_EX_XPSP1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIBICMPSTATS_EX_XPSP1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIBICMPSTATS_EX_XPSP1>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIBICMPSTATS_EX_XPSP1 {}
impl ::core::default::Default for MIBICMPSTATS_EX_XPSP1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_ANYCASTIPADDRESS_ROW {
    pub Address: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceLuid: NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub ScopeId: super::super::Networking::WinSock::SCOPE_ID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_ANYCASTIPADDRESS_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_ANYCASTIPADDRESS_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_ANYCASTIPADDRESS_ROW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_ANYCASTIPADDRESS_ROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_ANYCASTIPADDRESS_ROW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_ANYCASTIPADDRESS_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_ANYCASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_ANYCASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_ANYCASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_ANYCASTIPADDRESS_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_ANYCASTIPADDRESS_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_ANYCASTIPADDRESS_TABLE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_ANYCASTIPADDRESS_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_ANYCASTIPADDRESS_TABLE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_ANYCASTIPADDRESS_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_ANYCASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_BEST_IF {
    pub dwDestAddr: u32,
    pub dwIfIndex: u32,
}
impl ::core::marker::Copy for MIB_BEST_IF {}
impl ::core::clone::Clone for MIB_BEST_IF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_BEST_IF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_BEST_IF").field("dwDestAddr", &self.dwDestAddr).field("dwIfIndex", &self.dwIfIndex).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_BEST_IF {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_BEST_IF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_BEST_IF>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_BEST_IF {}
impl ::core::default::Default for MIB_BEST_IF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_BOUNDARYROW {
    pub dwGroupAddress: u32,
    pub dwGroupMask: u32,
}
impl ::core::marker::Copy for MIB_BOUNDARYROW {}
impl ::core::clone::Clone for MIB_BOUNDARYROW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_BOUNDARYROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_BOUNDARYROW").field("dwGroupAddress", &self.dwGroupAddress).field("dwGroupMask", &self.dwGroupMask).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_BOUNDARYROW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_BOUNDARYROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_BOUNDARYROW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_BOUNDARYROW {}
impl ::core::default::Default for MIB_BOUNDARYROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_ICMP {
    pub stats: MIBICMPINFO,
}
impl ::core::marker::Copy for MIB_ICMP {}
impl ::core::clone::Clone for MIB_ICMP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_ICMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_ICMP").field("stats", &self.stats).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_ICMP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_ICMP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_ICMP>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_ICMP {}
impl ::core::default::Default for MIB_ICMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_ICMP_EX_XPSP1 {
    pub icmpInStats: MIBICMPSTATS_EX_XPSP1,
    pub icmpOutStats: MIBICMPSTATS_EX_XPSP1,
}
impl ::core::marker::Copy for MIB_ICMP_EX_XPSP1 {}
impl ::core::clone::Clone for MIB_ICMP_EX_XPSP1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_ICMP_EX_XPSP1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_ICMP_EX_XPSP1").field("icmpInStats", &self.icmpInStats).field("icmpOutStats", &self.icmpOutStats).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_ICMP_EX_XPSP1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_ICMP_EX_XPSP1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_ICMP_EX_XPSP1>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_ICMP_EX_XPSP1 {}
impl ::core::default::Default for MIB_ICMP_EX_XPSP1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IFNUMBER {
    pub dwValue: u32,
}
impl ::core::marker::Copy for MIB_IFNUMBER {}
impl ::core::clone::Clone for MIB_IFNUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IFNUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IFNUMBER").field("dwValue", &self.dwValue).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IFNUMBER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IFNUMBER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IFNUMBER>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IFNUMBER {}
impl ::core::default::Default for MIB_IFNUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IFROW {
    pub wszName: [u16; 256],
    pub dwIndex: u32,
    pub dwType: u32,
    pub dwMtu: u32,
    pub dwSpeed: u32,
    pub dwPhysAddrLen: u32,
    pub bPhysAddr: [u8; 8],
    pub dwAdminStatus: u32,
    pub dwOperStatus: INTERNAL_IF_OPER_STATUS,
    pub dwLastChange: u32,
    pub dwInOctets: u32,
    pub dwInUcastPkts: u32,
    pub dwInNUcastPkts: u32,
    pub dwInDiscards: u32,
    pub dwInErrors: u32,
    pub dwInUnknownProtos: u32,
    pub dwOutOctets: u32,
    pub dwOutUcastPkts: u32,
    pub dwOutNUcastPkts: u32,
    pub dwOutDiscards: u32,
    pub dwOutErrors: u32,
    pub dwOutQLen: u32,
    pub dwDescrLen: u32,
    pub bDescr: [u8; 256],
}
impl ::core::marker::Copy for MIB_IFROW {}
impl ::core::clone::Clone for MIB_IFROW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IFROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IFROW")
            .field("wszName", &self.wszName)
            .field("dwIndex", &self.dwIndex)
            .field("dwType", &self.dwType)
            .field("dwMtu", &self.dwMtu)
            .field("dwSpeed", &self.dwSpeed)
            .field("dwPhysAddrLen", &self.dwPhysAddrLen)
            .field("bPhysAddr", &self.bPhysAddr)
            .field("dwAdminStatus", &self.dwAdminStatus)
            .field("dwOperStatus", &self.dwOperStatus)
            .field("dwLastChange", &self.dwLastChange)
            .field("dwInOctets", &self.dwInOctets)
            .field("dwInUcastPkts", &self.dwInUcastPkts)
            .field("dwInNUcastPkts", &self.dwInNUcastPkts)
            .field("dwInDiscards", &self.dwInDiscards)
            .field("dwInErrors", &self.dwInErrors)
            .field("dwInUnknownProtos", &self.dwInUnknownProtos)
            .field("dwOutOctets", &self.dwOutOctets)
            .field("dwOutUcastPkts", &self.dwOutUcastPkts)
            .field("dwOutNUcastPkts", &self.dwOutNUcastPkts)
            .field("dwOutDiscards", &self.dwOutDiscards)
            .field("dwOutErrors", &self.dwOutErrors)
            .field("dwOutQLen", &self.dwOutQLen)
            .field("dwDescrLen", &self.dwDescrLen)
            .field("bDescr", &self.bDescr)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IFROW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IFROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IFROW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IFROW {}
impl ::core::default::Default for MIB_IFROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IFSTACK_ROW {
    pub HigherLayerInterfaceIndex: u32,
    pub LowerLayerInterfaceIndex: u32,
}
impl ::core::marker::Copy for MIB_IFSTACK_ROW {}
impl ::core::clone::Clone for MIB_IFSTACK_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IFSTACK_ROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IFSTACK_ROW").field("HigherLayerInterfaceIndex", &self.HigherLayerInterfaceIndex).field("LowerLayerInterfaceIndex", &self.LowerLayerInterfaceIndex).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IFSTACK_ROW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IFSTACK_ROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IFSTACK_ROW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IFSTACK_ROW {}
impl ::core::default::Default for MIB_IFSTACK_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IFSTACK_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IFSTACK_ROW; 1],
}
impl ::core::marker::Copy for MIB_IFSTACK_TABLE {}
impl ::core::clone::Clone for MIB_IFSTACK_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IFSTACK_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IFSTACK_TABLE").field("NumEntries", &self.NumEntries).field("Table", &self.Table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IFSTACK_TABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IFSTACK_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IFSTACK_TABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IFSTACK_TABLE {}
impl ::core::default::Default for MIB_IFSTACK_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIB_IFSTATUS {
    pub dwIfIndex: u32,
    pub dwAdminStatus: u32,
    pub dwOperationalStatus: u32,
    pub bMHbeatActive: super::super::Foundation::BOOL,
    pub bMHbeatAlive: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIB_IFSTATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIB_IFSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MIB_IFSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IFSTATUS").field("dwIfIndex", &self.dwIfIndex).field("dwAdminStatus", &self.dwAdminStatus).field("dwOperationalStatus", &self.dwOperationalStatus).field("bMHbeatActive", &self.bMHbeatActive).field("bMHbeatAlive", &self.bMHbeatAlive).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIB_IFSTATUS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIB_IFSTATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IFSTATUS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIB_IFSTATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIB_IFSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IFTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IFROW; 1],
}
impl ::core::marker::Copy for MIB_IFTABLE {}
impl ::core::clone::Clone for MIB_IFTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IFTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IFTABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IFTABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IFTABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IFTABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IFTABLE {}
impl ::core::default::Default for MIB_IFTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_ADMIN_STATUS_DOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_ADMIN_STATUS_TESTING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_ADMIN_STATUS_UP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIB_IF_ENTRY_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibIfEntryNormal: MIB_IF_ENTRY_LEVEL = MIB_IF_ENTRY_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibIfEntryNormalWithoutStatistics: MIB_IF_ENTRY_LEVEL = MIB_IF_ENTRY_LEVEL(2i32);
impl ::core::marker::Copy for MIB_IF_ENTRY_LEVEL {}
impl ::core::clone::Clone for MIB_IF_ENTRY_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIB_IF_ENTRY_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MIB_IF_ENTRY_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MIB_IF_ENTRY_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_IF_ENTRY_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct MIB_IF_ROW2 {
    pub InterfaceLuid: NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub InterfaceGuid: ::windows::core::GUID,
    pub Alias: [u16; 257],
    pub Description: [u16; 257],
    pub PhysicalAddressLength: u32,
    pub PhysicalAddress: [u8; 32],
    pub PermanentPhysicalAddress: [u8; 32],
    pub Mtu: u32,
    pub Type: u32,
    pub TunnelType: TUNNEL_TYPE,
    pub MediaType: super::Ndis::NDIS_MEDIUM,
    pub PhysicalMediumType: super::Ndis::NDIS_PHYSICAL_MEDIUM,
    pub AccessType: NET_IF_ACCESS_TYPE,
    pub DirectionType: NET_IF_DIRECTION_TYPE,
    pub InterfaceAndOperStatusFlags: MIB_IF_ROW2_0,
    pub OperStatus: IF_OPER_STATUS,
    pub AdminStatus: NET_IF_ADMIN_STATUS,
    pub MediaConnectState: NET_IF_MEDIA_CONNECT_STATE,
    pub NetworkGuid: ::windows::core::GUID,
    pub ConnectionType: NET_IF_CONNECTION_TYPE,
    pub TransmitLinkSpeed: u64,
    pub ReceiveLinkSpeed: u64,
    pub InOctets: u64,
    pub InUcastPkts: u64,
    pub InNUcastPkts: u64,
    pub InDiscards: u64,
    pub InErrors: u64,
    pub InUnknownProtos: u64,
    pub InUcastOctets: u64,
    pub InMulticastOctets: u64,
    pub InBroadcastOctets: u64,
    pub OutOctets: u64,
    pub OutUcastPkts: u64,
    pub OutNUcastPkts: u64,
    pub OutDiscards: u64,
    pub OutErrors: u64,
    pub OutUcastOctets: u64,
    pub OutMulticastOctets: u64,
    pub OutBroadcastOctets: u64,
    pub OutQLen: u64,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for MIB_IF_ROW2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for MIB_IF_ROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for MIB_IF_ROW2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for MIB_IF_ROW2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IF_ROW2>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for MIB_IF_ROW2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for MIB_IF_ROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct MIB_IF_ROW2_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for MIB_IF_ROW2_0 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for MIB_IF_ROW2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for MIB_IF_ROW2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IF_ROW2_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for MIB_IF_ROW2_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for MIB_IF_ROW2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IF_ROW2_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for MIB_IF_ROW2_0 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for MIB_IF_ROW2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct MIB_IF_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IF_ROW2; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for MIB_IF_TABLE2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for MIB_IF_TABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for MIB_IF_TABLE2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for MIB_IF_TABLE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IF_TABLE2>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for MIB_IF_TABLE2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for MIB_IF_TABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIB_IF_TABLE_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibIfTableNormal: MIB_IF_TABLE_LEVEL = MIB_IF_TABLE_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibIfTableRaw: MIB_IF_TABLE_LEVEL = MIB_IF_TABLE_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibIfTableNormalWithoutStatistics: MIB_IF_TABLE_LEVEL = MIB_IF_TABLE_LEVEL(2i32);
impl ::core::marker::Copy for MIB_IF_TABLE_LEVEL {}
impl ::core::clone::Clone for MIB_IF_TABLE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIB_IF_TABLE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MIB_IF_TABLE_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MIB_IF_TABLE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_IF_TABLE_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_ETHERNET: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_FDDI: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_LOOPBACK: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_PPP: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_SLIP: u32 = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_TOKENRING: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_INVALID_TEREDO_PORT_NUMBER: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_INVERTEDIFSTACK_ROW {
    pub LowerLayerInterfaceIndex: u32,
    pub HigherLayerInterfaceIndex: u32,
}
impl ::core::marker::Copy for MIB_INVERTEDIFSTACK_ROW {}
impl ::core::clone::Clone for MIB_INVERTEDIFSTACK_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_INVERTEDIFSTACK_ROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_INVERTEDIFSTACK_ROW").field("LowerLayerInterfaceIndex", &self.LowerLayerInterfaceIndex).field("HigherLayerInterfaceIndex", &self.HigherLayerInterfaceIndex).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_INVERTEDIFSTACK_ROW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_INVERTEDIFSTACK_ROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_INVERTEDIFSTACK_ROW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_INVERTEDIFSTACK_ROW {}
impl ::core::default::Default for MIB_INVERTEDIFSTACK_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_INVERTEDIFSTACK_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_INVERTEDIFSTACK_ROW; 1],
}
impl ::core::marker::Copy for MIB_INVERTEDIFSTACK_TABLE {}
impl ::core::clone::Clone for MIB_INVERTEDIFSTACK_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_INVERTEDIFSTACK_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_INVERTEDIFSTACK_TABLE").field("NumEntries", &self.NumEntries).field("Table", &self.Table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_INVERTEDIFSTACK_TABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_INVERTEDIFSTACK_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_INVERTEDIFSTACK_TABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_INVERTEDIFSTACK_TABLE {}
impl ::core::default::Default for MIB_INVERTEDIFSTACK_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPADDRROW_W2K {
    pub dwAddr: u32,
    pub dwIndex: u32,
    pub dwMask: u32,
    pub dwBCastAddr: u32,
    pub dwReasmSize: u32,
    pub unused1: u16,
    pub unused2: u16,
}
impl ::core::marker::Copy for MIB_IPADDRROW_W2K {}
impl ::core::clone::Clone for MIB_IPADDRROW_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPADDRROW_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPADDRROW_W2K").field("dwAddr", &self.dwAddr).field("dwIndex", &self.dwIndex).field("dwMask", &self.dwMask).field("dwBCastAddr", &self.dwBCastAddr).field("dwReasmSize", &self.dwReasmSize).field("unused1", &self.unused1).field("unused2", &self.unused2).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPADDRROW_W2K {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPADDRROW_W2K {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPADDRROW_W2K>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPADDRROW_W2K {}
impl ::core::default::Default for MIB_IPADDRROW_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPADDRROW_XP {
    pub dwAddr: u32,
    pub dwIndex: u32,
    pub dwMask: u32,
    pub dwBCastAddr: u32,
    pub dwReasmSize: u32,
    pub unused1: u16,
    pub wType: u16,
}
impl ::core::marker::Copy for MIB_IPADDRROW_XP {}
impl ::core::clone::Clone for MIB_IPADDRROW_XP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPADDRROW_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPADDRROW_XP").field("dwAddr", &self.dwAddr).field("dwIndex", &self.dwIndex).field("dwMask", &self.dwMask).field("dwBCastAddr", &self.dwBCastAddr).field("dwReasmSize", &self.dwReasmSize).field("unused1", &self.unused1).field("wType", &self.wType).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPADDRROW_XP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPADDRROW_XP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPADDRROW_XP>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPADDRROW_XP {}
impl ::core::default::Default for MIB_IPADDRROW_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPADDRTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPADDRROW_XP; 1],
}
impl ::core::marker::Copy for MIB_IPADDRTABLE {}
impl ::core::clone::Clone for MIB_IPADDRTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPADDRTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPADDRTABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPADDRTABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPADDRTABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPADDRTABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPADDRTABLE {}
impl ::core::default::Default for MIB_IPADDRTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPADDR_DELETED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPADDR_DISCONNECTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPADDR_DNS_ELIGIBLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPADDR_DYNAMIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPADDR_PRIMARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPADDR_TRANSIENT: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_IPDESTROW {
    pub ForwardRow: MIB_IPFORWARDROW,
    pub dwForwardPreference: u32,
    pub dwForwardViewSet: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_IPDESTROW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_IPDESTROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MIB_IPDESTROW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MIB_IPDESTROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPDESTROW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MIB_IPDESTROW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_IPDESTROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_IPDESTTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPDESTROW; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_IPDESTTABLE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_IPDESTTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MIB_IPDESTTABLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MIB_IPDESTTABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPDESTTABLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MIB_IPDESTTABLE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_IPDESTTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPFORWARDNUMBER {
    pub dwValue: u32,
}
impl ::core::marker::Copy for MIB_IPFORWARDNUMBER {}
impl ::core::clone::Clone for MIB_IPFORWARDNUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPFORWARDNUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPFORWARDNUMBER").field("dwValue", &self.dwValue).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPFORWARDNUMBER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPFORWARDNUMBER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPFORWARDNUMBER>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPFORWARDNUMBER {}
impl ::core::default::Default for MIB_IPFORWARDNUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_IPFORWARDROW {
    pub dwForwardDest: u32,
    pub dwForwardMask: u32,
    pub dwForwardPolicy: u32,
    pub dwForwardNextHop: u32,
    pub dwForwardIfIndex: u32,
    pub Anonymous1: MIB_IPFORWARDROW_0,
    pub Anonymous2: MIB_IPFORWARDROW_1,
    pub dwForwardAge: u32,
    pub dwForwardNextHopAS: u32,
    pub dwForwardMetric1: u32,
    pub dwForwardMetric2: u32,
    pub dwForwardMetric3: u32,
    pub dwForwardMetric4: u32,
    pub dwForwardMetric5: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_IPFORWARDROW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_IPFORWARDROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MIB_IPFORWARDROW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MIB_IPFORWARDROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPFORWARDROW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MIB_IPFORWARDROW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_IPFORWARDROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union MIB_IPFORWARDROW_0 {
    pub dwForwardType: u32,
    pub ForwardType: MIB_IPFORWARD_TYPE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_IPFORWARDROW_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_IPFORWARDROW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MIB_IPFORWARDROW_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MIB_IPFORWARDROW_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPFORWARDROW_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MIB_IPFORWARDROW_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_IPFORWARDROW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union MIB_IPFORWARDROW_1 {
    pub dwForwardProto: u32,
    pub ForwardProto: super::super::Networking::WinSock::NL_ROUTE_PROTOCOL,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_IPFORWARDROW_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_IPFORWARDROW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MIB_IPFORWARDROW_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MIB_IPFORWARDROW_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPFORWARDROW_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MIB_IPFORWARDROW_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_IPFORWARDROW_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_IPFORWARDTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPFORWARDROW; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_IPFORWARDTABLE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_IPFORWARDTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MIB_IPFORWARDTABLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MIB_IPFORWARDTABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPFORWARDTABLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MIB_IPFORWARDTABLE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_IPFORWARDTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPFORWARD_ROW2 {
    pub InterfaceLuid: NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub DestinationPrefix: IP_ADDRESS_PREFIX,
    pub NextHop: super::super::Networking::WinSock::SOCKADDR_INET,
    pub SitePrefixLength: u8,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub Metric: u32,
    pub Protocol: super::super::Networking::WinSock::NL_ROUTE_PROTOCOL,
    pub Loopback: super::super::Foundation::BOOLEAN,
    pub AutoconfigureAddress: super::super::Foundation::BOOLEAN,
    pub Publish: super::super::Foundation::BOOLEAN,
    pub Immortal: super::super::Foundation::BOOLEAN,
    pub Age: u32,
    pub Origin: super::super::Networking::WinSock::NL_ROUTE_ORIGIN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPFORWARD_ROW2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPFORWARD_ROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IPFORWARD_ROW2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IPFORWARD_ROW2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPFORWARD_ROW2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IPFORWARD_ROW2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPFORWARD_ROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPFORWARD_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IPFORWARD_ROW2; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPFORWARD_TABLE2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPFORWARD_TABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IPFORWARD_TABLE2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IPFORWARD_TABLE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPFORWARD_TABLE2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IPFORWARD_TABLE2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPFORWARD_TABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIB_IPFORWARD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPROUTE_TYPE_OTHER: MIB_IPFORWARD_TYPE = MIB_IPFORWARD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPROUTE_TYPE_INVALID: MIB_IPFORWARD_TYPE = MIB_IPFORWARD_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPROUTE_TYPE_DIRECT: MIB_IPFORWARD_TYPE = MIB_IPFORWARD_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPROUTE_TYPE_INDIRECT: MIB_IPFORWARD_TYPE = MIB_IPFORWARD_TYPE(4i32);
impl ::core::marker::Copy for MIB_IPFORWARD_TYPE {}
impl ::core::clone::Clone for MIB_IPFORWARD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIB_IPFORWARD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MIB_IPFORWARD_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MIB_IPFORWARD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_IPFORWARD_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPINTERFACE_ROW {
    pub Family: u16,
    pub InterfaceLuid: NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub MaxReassemblySize: u32,
    pub InterfaceIdentifier: u64,
    pub MinRouterAdvertisementInterval: u32,
    pub MaxRouterAdvertisementInterval: u32,
    pub AdvertisingEnabled: super::super::Foundation::BOOLEAN,
    pub ForwardingEnabled: super::super::Foundation::BOOLEAN,
    pub WeakHostSend: super::super::Foundation::BOOLEAN,
    pub WeakHostReceive: super::super::Foundation::BOOLEAN,
    pub UseAutomaticMetric: super::super::Foundation::BOOLEAN,
    pub UseNeighborUnreachabilityDetection: super::super::Foundation::BOOLEAN,
    pub ManagedAddressConfigurationSupported: super::super::Foundation::BOOLEAN,
    pub OtherStatefulConfigurationSupported: super::super::Foundation::BOOLEAN,
    pub AdvertiseDefaultRoute: super::super::Foundation::BOOLEAN,
    pub RouterDiscoveryBehavior: super::super::Networking::WinSock::NL_ROUTER_DISCOVERY_BEHAVIOR,
    pub DadTransmits: u32,
    pub BaseReachableTime: u32,
    pub RetransmitTime: u32,
    pub PathMtuDiscoveryTimeout: u32,
    pub LinkLocalAddressBehavior: super::super::Networking::WinSock::NL_LINK_LOCAL_ADDRESS_BEHAVIOR,
    pub LinkLocalAddressTimeout: u32,
    pub ZoneIndices: [u32; 16],
    pub SitePrefixLength: u32,
    pub Metric: u32,
    pub NlMtu: u32,
    pub Connected: super::super::Foundation::BOOLEAN,
    pub SupportsWakeUpPatterns: super::super::Foundation::BOOLEAN,
    pub SupportsNeighborDiscovery: super::super::Foundation::BOOLEAN,
    pub SupportsRouterDiscovery: super::super::Foundation::BOOLEAN,
    pub ReachableTime: u32,
    pub TransmitOffload: super::super::Networking::WinSock::NL_INTERFACE_OFFLOAD_ROD,
    pub ReceiveOffload: super::super::Networking::WinSock::NL_INTERFACE_OFFLOAD_ROD,
    pub DisableDefaultRoutes: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPINTERFACE_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPINTERFACE_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IPINTERFACE_ROW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IPINTERFACE_ROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPINTERFACE_ROW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IPINTERFACE_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPINTERFACE_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPINTERFACE_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IPINTERFACE_ROW; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPINTERFACE_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPINTERFACE_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IPINTERFACE_TABLE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IPINTERFACE_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPINTERFACE_TABLE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IPINTERFACE_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPINTERFACE_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_BOUNDARY {
    pub dwIfIndex: u32,
    pub dwGroupAddress: u32,
    pub dwGroupMask: u32,
    pub dwStatus: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_BOUNDARY {}
impl ::core::clone::Clone for MIB_IPMCAST_BOUNDARY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_BOUNDARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_BOUNDARY").field("dwIfIndex", &self.dwIfIndex).field("dwGroupAddress", &self.dwGroupAddress).field("dwGroupMask", &self.dwGroupMask).field("dwStatus", &self.dwStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_BOUNDARY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_BOUNDARY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_BOUNDARY>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_BOUNDARY {}
impl ::core::default::Default for MIB_IPMCAST_BOUNDARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_BOUNDARY_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_BOUNDARY; 1],
}
impl ::core::marker::Copy for MIB_IPMCAST_BOUNDARY_TABLE {}
impl ::core::clone::Clone for MIB_IPMCAST_BOUNDARY_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_BOUNDARY_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_BOUNDARY_TABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_BOUNDARY_TABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_BOUNDARY_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_BOUNDARY_TABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_BOUNDARY_TABLE {}
impl ::core::default::Default for MIB_IPMCAST_BOUNDARY_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_GLOBAL {
    pub dwEnable: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_GLOBAL {}
impl ::core::clone::Clone for MIB_IPMCAST_GLOBAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_GLOBAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_GLOBAL").field("dwEnable", &self.dwEnable).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_GLOBAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_GLOBAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_GLOBAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_GLOBAL {}
impl ::core::default::Default for MIB_IPMCAST_GLOBAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_IF_ENTRY {
    pub dwIfIndex: u32,
    pub dwTtl: u32,
    pub dwProtocol: u32,
    pub dwRateLimit: u32,
    pub ulInMcastOctets: u32,
    pub ulOutMcastOctets: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_IF_ENTRY {}
impl ::core::clone::Clone for MIB_IPMCAST_IF_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_IF_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_IF_ENTRY").field("dwIfIndex", &self.dwIfIndex).field("dwTtl", &self.dwTtl).field("dwProtocol", &self.dwProtocol).field("dwRateLimit", &self.dwRateLimit).field("ulInMcastOctets", &self.ulInMcastOctets).field("ulOutMcastOctets", &self.ulOutMcastOctets).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_IF_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_IF_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_IF_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_IF_ENTRY {}
impl ::core::default::Default for MIB_IPMCAST_IF_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_IF_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_IF_ENTRY; 1],
}
impl ::core::marker::Copy for MIB_IPMCAST_IF_TABLE {}
impl ::core::clone::Clone for MIB_IPMCAST_IF_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_IF_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_IF_TABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_IF_TABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_IF_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_IF_TABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_IF_TABLE {}
impl ::core::default::Default for MIB_IPMCAST_IF_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_MFE {
    pub dwGroup: u32,
    pub dwSource: u32,
    pub dwSrcMask: u32,
    pub dwUpStrmNgbr: u32,
    pub dwInIfIndex: u32,
    pub dwInIfProtocol: u32,
    pub dwRouteProtocol: u32,
    pub dwRouteNetwork: u32,
    pub dwRouteMask: u32,
    pub ulUpTime: u32,
    pub ulExpiryTime: u32,
    pub ulTimeOut: u32,
    pub ulNumOutIf: u32,
    pub fFlags: u32,
    pub dwReserved: u32,
    pub rgmioOutInfo: [MIB_IPMCAST_OIF_XP; 1],
}
impl ::core::marker::Copy for MIB_IPMCAST_MFE {}
impl ::core::clone::Clone for MIB_IPMCAST_MFE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_MFE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_MFE")
            .field("dwGroup", &self.dwGroup)
            .field("dwSource", &self.dwSource)
            .field("dwSrcMask", &self.dwSrcMask)
            .field("dwUpStrmNgbr", &self.dwUpStrmNgbr)
            .field("dwInIfIndex", &self.dwInIfIndex)
            .field("dwInIfProtocol", &self.dwInIfProtocol)
            .field("dwRouteProtocol", &self.dwRouteProtocol)
            .field("dwRouteNetwork", &self.dwRouteNetwork)
            .field("dwRouteMask", &self.dwRouteMask)
            .field("ulUpTime", &self.ulUpTime)
            .field("ulExpiryTime", &self.ulExpiryTime)
            .field("ulTimeOut", &self.ulTimeOut)
            .field("ulNumOutIf", &self.ulNumOutIf)
            .field("fFlags", &self.fFlags)
            .field("dwReserved", &self.dwReserved)
            .field("rgmioOutInfo", &self.rgmioOutInfo)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_MFE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_MFE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_MFE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_MFE {}
impl ::core::default::Default for MIB_IPMCAST_MFE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_MFE_STATS {
    pub dwGroup: u32,
    pub dwSource: u32,
    pub dwSrcMask: u32,
    pub dwUpStrmNgbr: u32,
    pub dwInIfIndex: u32,
    pub dwInIfProtocol: u32,
    pub dwRouteProtocol: u32,
    pub dwRouteNetwork: u32,
    pub dwRouteMask: u32,
    pub ulUpTime: u32,
    pub ulExpiryTime: u32,
    pub ulNumOutIf: u32,
    pub ulInPkts: u32,
    pub ulInOctets: u32,
    pub ulPktsDifferentIf: u32,
    pub ulQueueOverflow: u32,
    pub rgmiosOutStats: [MIB_IPMCAST_OIF_STATS_LH; 1],
}
impl ::core::marker::Copy for MIB_IPMCAST_MFE_STATS {}
impl ::core::clone::Clone for MIB_IPMCAST_MFE_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_MFE_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_MFE_STATS")
            .field("dwGroup", &self.dwGroup)
            .field("dwSource", &self.dwSource)
            .field("dwSrcMask", &self.dwSrcMask)
            .field("dwUpStrmNgbr", &self.dwUpStrmNgbr)
            .field("dwInIfIndex", &self.dwInIfIndex)
            .field("dwInIfProtocol", &self.dwInIfProtocol)
            .field("dwRouteProtocol", &self.dwRouteProtocol)
            .field("dwRouteNetwork", &self.dwRouteNetwork)
            .field("dwRouteMask", &self.dwRouteMask)
            .field("ulUpTime", &self.ulUpTime)
            .field("ulExpiryTime", &self.ulExpiryTime)
            .field("ulNumOutIf", &self.ulNumOutIf)
            .field("ulInPkts", &self.ulInPkts)
            .field("ulInOctets", &self.ulInOctets)
            .field("ulPktsDifferentIf", &self.ulPktsDifferentIf)
            .field("ulQueueOverflow", &self.ulQueueOverflow)
            .field("rgmiosOutStats", &self.rgmiosOutStats)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_MFE_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_MFE_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_MFE_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_MFE_STATS {}
impl ::core::default::Default for MIB_IPMCAST_MFE_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_MFE_STATS_EX_XP {
    pub dwGroup: u32,
    pub dwSource: u32,
    pub dwSrcMask: u32,
    pub dwUpStrmNgbr: u32,
    pub dwInIfIndex: u32,
    pub dwInIfProtocol: u32,
    pub dwRouteProtocol: u32,
    pub dwRouteNetwork: u32,
    pub dwRouteMask: u32,
    pub ulUpTime: u32,
    pub ulExpiryTime: u32,
    pub ulNumOutIf: u32,
    pub ulInPkts: u32,
    pub ulInOctets: u32,
    pub ulPktsDifferentIf: u32,
    pub ulQueueOverflow: u32,
    pub ulUninitMfe: u32,
    pub ulNegativeMfe: u32,
    pub ulInDiscards: u32,
    pub ulInHdrErrors: u32,
    pub ulTotalOutPackets: u32,
    pub rgmiosOutStats: [MIB_IPMCAST_OIF_STATS_LH; 1],
}
impl ::core::marker::Copy for MIB_IPMCAST_MFE_STATS_EX_XP {}
impl ::core::clone::Clone for MIB_IPMCAST_MFE_STATS_EX_XP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_MFE_STATS_EX_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_MFE_STATS_EX_XP")
            .field("dwGroup", &self.dwGroup)
            .field("dwSource", &self.dwSource)
            .field("dwSrcMask", &self.dwSrcMask)
            .field("dwUpStrmNgbr", &self.dwUpStrmNgbr)
            .field("dwInIfIndex", &self.dwInIfIndex)
            .field("dwInIfProtocol", &self.dwInIfProtocol)
            .field("dwRouteProtocol", &self.dwRouteProtocol)
            .field("dwRouteNetwork", &self.dwRouteNetwork)
            .field("dwRouteMask", &self.dwRouteMask)
            .field("ulUpTime", &self.ulUpTime)
            .field("ulExpiryTime", &self.ulExpiryTime)
            .field("ulNumOutIf", &self.ulNumOutIf)
            .field("ulInPkts", &self.ulInPkts)
            .field("ulInOctets", &self.ulInOctets)
            .field("ulPktsDifferentIf", &self.ulPktsDifferentIf)
            .field("ulQueueOverflow", &self.ulQueueOverflow)
            .field("ulUninitMfe", &self.ulUninitMfe)
            .field("ulNegativeMfe", &self.ulNegativeMfe)
            .field("ulInDiscards", &self.ulInDiscards)
            .field("ulInHdrErrors", &self.ulInHdrErrors)
            .field("ulTotalOutPackets", &self.ulTotalOutPackets)
            .field("rgmiosOutStats", &self.rgmiosOutStats)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_MFE_STATS_EX_XP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_MFE_STATS_EX_XP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_MFE_STATS_EX_XP>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_MFE_STATS_EX_XP {}
impl ::core::default::Default for MIB_IPMCAST_MFE_STATS_EX_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_OIF_STATS_LH {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub dwDialContext: u32,
    pub ulTtlTooLow: u32,
    pub ulFragNeeded: u32,
    pub ulOutPackets: u32,
    pub ulOutDiscards: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_OIF_STATS_LH {}
impl ::core::clone::Clone for MIB_IPMCAST_OIF_STATS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_OIF_STATS_LH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_OIF_STATS_LH").field("dwOutIfIndex", &self.dwOutIfIndex).field("dwNextHopAddr", &self.dwNextHopAddr).field("dwDialContext", &self.dwDialContext).field("ulTtlTooLow", &self.ulTtlTooLow).field("ulFragNeeded", &self.ulFragNeeded).field("ulOutPackets", &self.ulOutPackets).field("ulOutDiscards", &self.ulOutDiscards).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_OIF_STATS_LH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_OIF_STATS_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_OIF_STATS_LH>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_OIF_STATS_LH {}
impl ::core::default::Default for MIB_IPMCAST_OIF_STATS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_OIF_STATS_W2K {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub pvDialContext: *mut ::core::ffi::c_void,
    pub ulTtlTooLow: u32,
    pub ulFragNeeded: u32,
    pub ulOutPackets: u32,
    pub ulOutDiscards: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_OIF_STATS_W2K {}
impl ::core::clone::Clone for MIB_IPMCAST_OIF_STATS_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_OIF_STATS_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_OIF_STATS_W2K").field("dwOutIfIndex", &self.dwOutIfIndex).field("dwNextHopAddr", &self.dwNextHopAddr).field("pvDialContext", &self.pvDialContext).field("ulTtlTooLow", &self.ulTtlTooLow).field("ulFragNeeded", &self.ulFragNeeded).field("ulOutPackets", &self.ulOutPackets).field("ulOutDiscards", &self.ulOutDiscards).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_OIF_STATS_W2K {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_OIF_STATS_W2K {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_OIF_STATS_W2K>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_OIF_STATS_W2K {}
impl ::core::default::Default for MIB_IPMCAST_OIF_STATS_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_OIF_W2K {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_OIF_W2K {}
impl ::core::clone::Clone for MIB_IPMCAST_OIF_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_OIF_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_OIF_W2K").field("dwOutIfIndex", &self.dwOutIfIndex).field("dwNextHopAddr", &self.dwNextHopAddr).field("pvReserved", &self.pvReserved).field("dwReserved", &self.dwReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_OIF_W2K {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_OIF_W2K {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_OIF_W2K>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_OIF_W2K {}
impl ::core::default::Default for MIB_IPMCAST_OIF_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_OIF_XP {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub dwReserved: u32,
    pub dwReserved1: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_OIF_XP {}
impl ::core::clone::Clone for MIB_IPMCAST_OIF_XP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_OIF_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_OIF_XP").field("dwOutIfIndex", &self.dwOutIfIndex).field("dwNextHopAddr", &self.dwNextHopAddr).field("dwReserved", &self.dwReserved).field("dwReserved1", &self.dwReserved1).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_OIF_XP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_OIF_XP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_OIF_XP>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_OIF_XP {}
impl ::core::default::Default for MIB_IPMCAST_OIF_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_SCOPE {
    pub dwGroupAddress: u32,
    pub dwGroupMask: u32,
    pub snNameBuffer: [u16; 256],
    pub dwStatus: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_SCOPE {}
impl ::core::clone::Clone for MIB_IPMCAST_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPMCAST_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_SCOPE").field("dwGroupAddress", &self.dwGroupAddress).field("dwGroupMask", &self.dwGroupMask).field("snNameBuffer", &self.snNameBuffer).field("dwStatus", &self.dwStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPMCAST_SCOPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_SCOPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPMCAST_SCOPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_SCOPE {}
impl ::core::default::Default for MIB_IPMCAST_SCOPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPNETROW_LH {
    pub dwIndex: u32,
    pub dwPhysAddrLen: u32,
    pub bPhysAddr: [u8; 8],
    pub dwAddr: u32,
    pub Anonymous: MIB_IPNETROW_LH_0,
}
impl ::core::marker::Copy for MIB_IPNETROW_LH {}
impl ::core::clone::Clone for MIB_IPNETROW_LH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_IPNETROW_LH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPNETROW_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPNETROW_LH>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPNETROW_LH {}
impl ::core::default::Default for MIB_IPNETROW_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_IPNETROW_LH_0 {
    pub dwType: u32,
    pub Type: MIB_IPNET_TYPE,
}
impl ::core::marker::Copy for MIB_IPNETROW_LH_0 {}
impl ::core::clone::Clone for MIB_IPNETROW_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_IPNETROW_LH_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPNETROW_LH_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPNETROW_LH_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPNETROW_LH_0 {}
impl ::core::default::Default for MIB_IPNETROW_LH_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPNETROW_W2K {
    pub dwIndex: u32,
    pub dwPhysAddrLen: u32,
    pub bPhysAddr: [u8; 8],
    pub dwAddr: u32,
    pub dwType: u32,
}
impl ::core::marker::Copy for MIB_IPNETROW_W2K {}
impl ::core::clone::Clone for MIB_IPNETROW_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPNETROW_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPNETROW_W2K").field("dwIndex", &self.dwIndex).field("dwPhysAddrLen", &self.dwPhysAddrLen).field("bPhysAddr", &self.bPhysAddr).field("dwAddr", &self.dwAddr).field("dwType", &self.dwType).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPNETROW_W2K {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPNETROW_W2K {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPNETROW_W2K>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPNETROW_W2K {}
impl ::core::default::Default for MIB_IPNETROW_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPNETTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPNETROW_LH; 1],
}
impl ::core::marker::Copy for MIB_IPNETTABLE {}
impl ::core::clone::Clone for MIB_IPNETTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_IPNETTABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPNETTABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPNETTABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPNETTABLE {}
impl ::core::default::Default for MIB_IPNETTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPNET_ROW2 {
    pub Address: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceIndex: u32,
    pub InterfaceLuid: NET_LUID_LH,
    pub PhysicalAddress: [u8; 32],
    pub PhysicalAddressLength: u32,
    pub State: super::super::Networking::WinSock::NL_NEIGHBOR_STATE,
    pub Anonymous: MIB_IPNET_ROW2_0,
    pub ReachabilityTime: MIB_IPNET_ROW2_1,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPNET_ROW2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPNET_ROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IPNET_ROW2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IPNET_ROW2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPNET_ROW2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IPNET_ROW2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPNET_ROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union MIB_IPNET_ROW2_0 {
    pub Anonymous: MIB_IPNET_ROW2_0_0,
    pub Flags: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPNET_ROW2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPNET_ROW2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IPNET_ROW2_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IPNET_ROW2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPNET_ROW2_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IPNET_ROW2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPNET_ROW2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPNET_ROW2_0_0 {
    pub _bitfield: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPNET_ROW2_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPNET_ROW2_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for MIB_IPNET_ROW2_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPNET_ROW2_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IPNET_ROW2_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IPNET_ROW2_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPNET_ROW2_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IPNET_ROW2_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPNET_ROW2_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union MIB_IPNET_ROW2_1 {
    pub LastReachable: u32,
    pub LastUnreachable: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPNET_ROW2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPNET_ROW2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IPNET_ROW2_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IPNET_ROW2_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPNET_ROW2_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IPNET_ROW2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPNET_ROW2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPNET_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IPNET_ROW2; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPNET_TABLE2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPNET_TABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IPNET_TABLE2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IPNET_TABLE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPNET_TABLE2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IPNET_TABLE2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPNET_TABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIB_IPNET_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPNET_TYPE_OTHER: MIB_IPNET_TYPE = MIB_IPNET_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPNET_TYPE_INVALID: MIB_IPNET_TYPE = MIB_IPNET_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPNET_TYPE_DYNAMIC: MIB_IPNET_TYPE = MIB_IPNET_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPNET_TYPE_STATIC: MIB_IPNET_TYPE = MIB_IPNET_TYPE(4i32);
impl ::core::marker::Copy for MIB_IPNET_TYPE {}
impl ::core::clone::Clone for MIB_IPNET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIB_IPNET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MIB_IPNET_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MIB_IPNET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_IPNET_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPPATH_ROW {
    pub Source: super::super::Networking::WinSock::SOCKADDR_INET,
    pub Destination: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceLuid: NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub CurrentNextHop: super::super::Networking::WinSock::SOCKADDR_INET,
    pub PathMtu: u32,
    pub RttMean: u32,
    pub RttDeviation: u32,
    pub Anonymous: MIB_IPPATH_ROW_0,
    pub IsReachable: super::super::Foundation::BOOLEAN,
    pub LinkTransmitSpeed: u64,
    pub LinkReceiveSpeed: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPPATH_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPPATH_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IPPATH_ROW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IPPATH_ROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPPATH_ROW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IPPATH_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPPATH_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union MIB_IPPATH_ROW_0 {
    pub LastReachable: u32,
    pub LastUnreachable: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPPATH_ROW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPPATH_ROW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IPPATH_ROW_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IPPATH_ROW_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPPATH_ROW_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IPPATH_ROW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPPATH_ROW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPPATH_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IPPATH_ROW; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPPATH_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPPATH_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IPPATH_TABLE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IPPATH_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPPATH_TABLE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IPPATH_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPPATH_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPROUTE_METRIC_UNUSED: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIB_IPSTATS_FORWARDING(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IP_FORWARDING: MIB_IPSTATS_FORWARDING = MIB_IPSTATS_FORWARDING(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IP_NOT_FORWARDING: MIB_IPSTATS_FORWARDING = MIB_IPSTATS_FORWARDING(2i32);
impl ::core::marker::Copy for MIB_IPSTATS_FORWARDING {}
impl ::core::clone::Clone for MIB_IPSTATS_FORWARDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIB_IPSTATS_FORWARDING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MIB_IPSTATS_FORWARDING {
    type Abi = Self;
}
impl ::core::fmt::Debug for MIB_IPSTATS_FORWARDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_IPSTATS_FORWARDING").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPSTATS_LH {
    pub Anonymous: MIB_IPSTATS_LH_0,
    pub dwDefaultTTL: u32,
    pub dwInReceives: u32,
    pub dwInHdrErrors: u32,
    pub dwInAddrErrors: u32,
    pub dwForwDatagrams: u32,
    pub dwInUnknownProtos: u32,
    pub dwInDiscards: u32,
    pub dwInDelivers: u32,
    pub dwOutRequests: u32,
    pub dwRoutingDiscards: u32,
    pub dwOutDiscards: u32,
    pub dwOutNoRoutes: u32,
    pub dwReasmTimeout: u32,
    pub dwReasmReqds: u32,
    pub dwReasmOks: u32,
    pub dwReasmFails: u32,
    pub dwFragOks: u32,
    pub dwFragFails: u32,
    pub dwFragCreates: u32,
    pub dwNumIf: u32,
    pub dwNumAddr: u32,
    pub dwNumRoutes: u32,
}
impl ::core::marker::Copy for MIB_IPSTATS_LH {}
impl ::core::clone::Clone for MIB_IPSTATS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_IPSTATS_LH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPSTATS_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPSTATS_LH>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPSTATS_LH {}
impl ::core::default::Default for MIB_IPSTATS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_IPSTATS_LH_0 {
    pub dwForwarding: u32,
    pub Forwarding: MIB_IPSTATS_FORWARDING,
}
impl ::core::marker::Copy for MIB_IPSTATS_LH_0 {}
impl ::core::clone::Clone for MIB_IPSTATS_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_IPSTATS_LH_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPSTATS_LH_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPSTATS_LH_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPSTATS_LH_0 {}
impl ::core::default::Default for MIB_IPSTATS_LH_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPSTATS_W2K {
    pub dwForwarding: u32,
    pub dwDefaultTTL: u32,
    pub dwInReceives: u32,
    pub dwInHdrErrors: u32,
    pub dwInAddrErrors: u32,
    pub dwForwDatagrams: u32,
    pub dwInUnknownProtos: u32,
    pub dwInDiscards: u32,
    pub dwInDelivers: u32,
    pub dwOutRequests: u32,
    pub dwRoutingDiscards: u32,
    pub dwOutDiscards: u32,
    pub dwOutNoRoutes: u32,
    pub dwReasmTimeout: u32,
    pub dwReasmReqds: u32,
    pub dwReasmOks: u32,
    pub dwReasmFails: u32,
    pub dwFragOks: u32,
    pub dwFragFails: u32,
    pub dwFragCreates: u32,
    pub dwNumIf: u32,
    pub dwNumAddr: u32,
    pub dwNumRoutes: u32,
}
impl ::core::marker::Copy for MIB_IPSTATS_W2K {}
impl ::core::clone::Clone for MIB_IPSTATS_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_IPSTATS_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPSTATS_W2K")
            .field("dwForwarding", &self.dwForwarding)
            .field("dwDefaultTTL", &self.dwDefaultTTL)
            .field("dwInReceives", &self.dwInReceives)
            .field("dwInHdrErrors", &self.dwInHdrErrors)
            .field("dwInAddrErrors", &self.dwInAddrErrors)
            .field("dwForwDatagrams", &self.dwForwDatagrams)
            .field("dwInUnknownProtos", &self.dwInUnknownProtos)
            .field("dwInDiscards", &self.dwInDiscards)
            .field("dwInDelivers", &self.dwInDelivers)
            .field("dwOutRequests", &self.dwOutRequests)
            .field("dwRoutingDiscards", &self.dwRoutingDiscards)
            .field("dwOutDiscards", &self.dwOutDiscards)
            .field("dwOutNoRoutes", &self.dwOutNoRoutes)
            .field("dwReasmTimeout", &self.dwReasmTimeout)
            .field("dwReasmReqds", &self.dwReasmReqds)
            .field("dwReasmOks", &self.dwReasmOks)
            .field("dwReasmFails", &self.dwReasmFails)
            .field("dwFragOks", &self.dwFragOks)
            .field("dwFragFails", &self.dwFragFails)
            .field("dwFragCreates", &self.dwFragCreates)
            .field("dwNumIf", &self.dwNumIf)
            .field("dwNumAddr", &self.dwNumAddr)
            .field("dwNumRoutes", &self.dwNumRoutes)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_IPSTATS_W2K {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_IPSTATS_W2K {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IPSTATS_W2K>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_IPSTATS_W2K {}
impl ::core::default::Default for MIB_IPSTATS_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    pub InboundBandwidthInformation: super::super::Networking::WinSock::NL_BANDWIDTH_INFORMATION,
    pub OutboundBandwidthInformation: super::super::Networking::WinSock::NL_BANDWIDTH_INFORMATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES").field("InboundBandwidthInformation", &self.InboundBandwidthInformation).field("OutboundBandwidthInformation", &self.OutboundBandwidthInformation).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_MCAST_LIMIT_ROW {
    pub dwTtl: u32,
    pub dwRateLimit: u32,
}
impl ::core::marker::Copy for MIB_MCAST_LIMIT_ROW {}
impl ::core::clone::Clone for MIB_MCAST_LIMIT_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_MCAST_LIMIT_ROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_MCAST_LIMIT_ROW").field("dwTtl", &self.dwTtl).field("dwRateLimit", &self.dwRateLimit).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_MCAST_LIMIT_ROW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_MCAST_LIMIT_ROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_MCAST_LIMIT_ROW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_MCAST_LIMIT_ROW {}
impl ::core::default::Default for MIB_MCAST_LIMIT_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_MFE_STATS_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_MFE_STATS; 1],
}
impl ::core::marker::Copy for MIB_MFE_STATS_TABLE {}
impl ::core::clone::Clone for MIB_MFE_STATS_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_MFE_STATS_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_MFE_STATS_TABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_MFE_STATS_TABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_MFE_STATS_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_MFE_STATS_TABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_MFE_STATS_TABLE {}
impl ::core::default::Default for MIB_MFE_STATS_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_MFE_STATS_TABLE_EX_XP {
    pub dwNumEntries: u32,
    pub table: [*mut MIB_IPMCAST_MFE_STATS_EX_XP; 1],
}
impl ::core::marker::Copy for MIB_MFE_STATS_TABLE_EX_XP {}
impl ::core::clone::Clone for MIB_MFE_STATS_TABLE_EX_XP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_MFE_STATS_TABLE_EX_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_MFE_STATS_TABLE_EX_XP").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_MFE_STATS_TABLE_EX_XP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_MFE_STATS_TABLE_EX_XP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_MFE_STATS_TABLE_EX_XP>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_MFE_STATS_TABLE_EX_XP {}
impl ::core::default::Default for MIB_MFE_STATS_TABLE_EX_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_MFE_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_MFE; 1],
}
impl ::core::marker::Copy for MIB_MFE_TABLE {}
impl ::core::clone::Clone for MIB_MFE_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_MFE_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_MFE_TABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_MFE_TABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_MFE_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_MFE_TABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_MFE_TABLE {}
impl ::core::default::Default for MIB_MFE_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_MULTICASTIPADDRESS_ROW {
    pub Address: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceIndex: u32,
    pub InterfaceLuid: NET_LUID_LH,
    pub ScopeId: super::super::Networking::WinSock::SCOPE_ID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_MULTICASTIPADDRESS_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_MULTICASTIPADDRESS_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_MULTICASTIPADDRESS_ROW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_MULTICASTIPADDRESS_ROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_MULTICASTIPADDRESS_ROW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_MULTICASTIPADDRESS_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_MULTICASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_MULTICASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_MULTICASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_MULTICASTIPADDRESS_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_MULTICASTIPADDRESS_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_MULTICASTIPADDRESS_TABLE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_MULTICASTIPADDRESS_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_MULTICASTIPADDRESS_TABLE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_MULTICASTIPADDRESS_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_MULTICASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIB_NOTIFICATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibParameterNotification: MIB_NOTIFICATION_TYPE = MIB_NOTIFICATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibAddInstance: MIB_NOTIFICATION_TYPE = MIB_NOTIFICATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibDeleteInstance: MIB_NOTIFICATION_TYPE = MIB_NOTIFICATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibInitialNotification: MIB_NOTIFICATION_TYPE = MIB_NOTIFICATION_TYPE(3i32);
impl ::core::marker::Copy for MIB_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for MIB_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIB_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MIB_NOTIFICATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MIB_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_OPAQUE_INFO {
    pub dwId: u32,
    pub Anonymous: MIB_OPAQUE_INFO_0,
}
impl ::core::marker::Copy for MIB_OPAQUE_INFO {}
impl ::core::clone::Clone for MIB_OPAQUE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_OPAQUE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_OPAQUE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_OPAQUE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_OPAQUE_INFO {}
impl ::core::default::Default for MIB_OPAQUE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_OPAQUE_INFO_0 {
    pub ullAlign: u64,
    pub rgbyData: [u8; 1],
}
impl ::core::marker::Copy for MIB_OPAQUE_INFO_0 {}
impl ::core::clone::Clone for MIB_OPAQUE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_OPAQUE_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_OPAQUE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_OPAQUE_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_OPAQUE_INFO_0 {}
impl ::core::default::Default for MIB_OPAQUE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_OPAQUE_QUERY {
    pub dwVarId: u32,
    pub rgdwVarIndex: [u32; 1],
}
impl ::core::marker::Copy for MIB_OPAQUE_QUERY {}
impl ::core::clone::Clone for MIB_OPAQUE_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_OPAQUE_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_OPAQUE_QUERY").field("dwVarId", &self.dwVarId).field("rgdwVarIndex", &self.rgdwVarIndex).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_OPAQUE_QUERY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_OPAQUE_QUERY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_OPAQUE_QUERY>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_OPAQUE_QUERY {}
impl ::core::default::Default for MIB_OPAQUE_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_PROXYARP {
    pub dwAddress: u32,
    pub dwMask: u32,
    pub dwIfIndex: u32,
}
impl ::core::marker::Copy for MIB_PROXYARP {}
impl ::core::clone::Clone for MIB_PROXYARP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_PROXYARP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_PROXYARP").field("dwAddress", &self.dwAddress).field("dwMask", &self.dwMask).field("dwIfIndex", &self.dwIfIndex).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_PROXYARP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_PROXYARP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_PROXYARP>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_PROXYARP {}
impl ::core::default::Default for MIB_PROXYARP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIB_ROUTESTATE {
    pub bRoutesSetToStack: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIB_ROUTESTATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIB_ROUTESTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MIB_ROUTESTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_ROUTESTATE").field("bRoutesSetToStack", &self.bRoutesSetToStack).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIB_ROUTESTATE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIB_ROUTESTATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_ROUTESTATE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIB_ROUTESTATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIB_ROUTESTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_TCP6ROW {
    pub State: MIB_TCP_STATE,
    pub LocalAddr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub RemoteAddr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_TCP6ROW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_TCP6ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MIB_TCP6ROW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MIB_TCP6ROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCP6ROW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MIB_TCP6ROW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_TCP6ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_TCP6ROW2 {
    pub LocalAddr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub RemoteAddr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
    pub State: MIB_TCP_STATE,
    pub dwOwningPid: u32,
    pub dwOffloadState: TCP_CONNECTION_OFFLOAD_STATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_TCP6ROW2 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_TCP6ROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MIB_TCP6ROW2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MIB_TCP6ROW2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCP6ROW2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MIB_TCP6ROW2 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_TCP6ROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCP6ROW_OWNER_MODULE {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub ucRemoteAddr: [u8; 16],
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
    pub dwState: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub OwningModuleInfo: [u64; 16],
}
impl ::core::marker::Copy for MIB_TCP6ROW_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_TCP6ROW_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCP6ROW_OWNER_MODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCP6ROW_OWNER_MODULE")
            .field("ucLocalAddr", &self.ucLocalAddr)
            .field("dwLocalScopeId", &self.dwLocalScopeId)
            .field("dwLocalPort", &self.dwLocalPort)
            .field("ucRemoteAddr", &self.ucRemoteAddr)
            .field("dwRemoteScopeId", &self.dwRemoteScopeId)
            .field("dwRemotePort", &self.dwRemotePort)
            .field("dwState", &self.dwState)
            .field("dwOwningPid", &self.dwOwningPid)
            .field("liCreateTimestamp", &self.liCreateTimestamp)
            .field("OwningModuleInfo", &self.OwningModuleInfo)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCP6ROW_OWNER_MODULE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCP6ROW_OWNER_MODULE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCP6ROW_OWNER_MODULE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCP6ROW_OWNER_MODULE {}
impl ::core::default::Default for MIB_TCP6ROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCP6ROW_OWNER_PID {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub ucRemoteAddr: [u8; 16],
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
    pub dwState: u32,
    pub dwOwningPid: u32,
}
impl ::core::marker::Copy for MIB_TCP6ROW_OWNER_PID {}
impl ::core::clone::Clone for MIB_TCP6ROW_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCP6ROW_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCP6ROW_OWNER_PID").field("ucLocalAddr", &self.ucLocalAddr).field("dwLocalScopeId", &self.dwLocalScopeId).field("dwLocalPort", &self.dwLocalPort).field("ucRemoteAddr", &self.ucRemoteAddr).field("dwRemoteScopeId", &self.dwRemoteScopeId).field("dwRemotePort", &self.dwRemotePort).field("dwState", &self.dwState).field("dwOwningPid", &self.dwOwningPid).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCP6ROW_OWNER_PID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCP6ROW_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCP6ROW_OWNER_PID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCP6ROW_OWNER_PID {}
impl ::core::default::Default for MIB_TCP6ROW_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_TCP6TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_TCP6TABLE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_TCP6TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MIB_TCP6TABLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MIB_TCP6TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCP6TABLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MIB_TCP6TABLE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_TCP6TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_TCP6TABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW2; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_TCP6TABLE2 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_TCP6TABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MIB_TCP6TABLE2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MIB_TCP6TABLE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCP6TABLE2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MIB_TCP6TABLE2 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_TCP6TABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCP6TABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW_OWNER_MODULE; 1],
}
impl ::core::marker::Copy for MIB_TCP6TABLE_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_TCP6TABLE_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCP6TABLE_OWNER_MODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCP6TABLE_OWNER_MODULE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCP6TABLE_OWNER_MODULE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCP6TABLE_OWNER_MODULE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCP6TABLE_OWNER_MODULE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCP6TABLE_OWNER_MODULE {}
impl ::core::default::Default for MIB_TCP6TABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCP6TABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW_OWNER_PID; 1],
}
impl ::core::marker::Copy for MIB_TCP6TABLE_OWNER_PID {}
impl ::core::clone::Clone for MIB_TCP6TABLE_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCP6TABLE_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCP6TABLE_OWNER_PID").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCP6TABLE_OWNER_PID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCP6TABLE_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCP6TABLE_OWNER_PID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCP6TABLE_OWNER_PID {}
impl ::core::default::Default for MIB_TCP6TABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPROW2 {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
    pub dwOwningPid: u32,
    pub dwOffloadState: TCP_CONNECTION_OFFLOAD_STATE,
}
impl ::core::marker::Copy for MIB_TCPROW2 {}
impl ::core::clone::Clone for MIB_TCPROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCPROW2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPROW2").field("dwState", &self.dwState).field("dwLocalAddr", &self.dwLocalAddr).field("dwLocalPort", &self.dwLocalPort).field("dwRemoteAddr", &self.dwRemoteAddr).field("dwRemotePort", &self.dwRemotePort).field("dwOwningPid", &self.dwOwningPid).field("dwOffloadState", &self.dwOffloadState).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPROW2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPROW2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPROW2>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPROW2 {}
impl ::core::default::Default for MIB_TCPROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPROW_LH {
    pub Anonymous: MIB_TCPROW_LH_0,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
}
impl ::core::marker::Copy for MIB_TCPROW_LH {}
impl ::core::clone::Clone for MIB_TCPROW_LH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPROW_LH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPROW_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPROW_LH>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPROW_LH {}
impl ::core::default::Default for MIB_TCPROW_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_TCPROW_LH_0 {
    pub dwState: u32,
    pub State: MIB_TCP_STATE,
}
impl ::core::marker::Copy for MIB_TCPROW_LH_0 {}
impl ::core::clone::Clone for MIB_TCPROW_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPROW_LH_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPROW_LH_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPROW_LH_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPROW_LH_0 {}
impl ::core::default::Default for MIB_TCPROW_LH_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPROW_OWNER_MODULE {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub OwningModuleInfo: [u64; 16],
}
impl ::core::marker::Copy for MIB_TCPROW_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_TCPROW_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCPROW_OWNER_MODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPROW_OWNER_MODULE").field("dwState", &self.dwState).field("dwLocalAddr", &self.dwLocalAddr).field("dwLocalPort", &self.dwLocalPort).field("dwRemoteAddr", &self.dwRemoteAddr).field("dwRemotePort", &self.dwRemotePort).field("dwOwningPid", &self.dwOwningPid).field("liCreateTimestamp", &self.liCreateTimestamp).field("OwningModuleInfo", &self.OwningModuleInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPROW_OWNER_MODULE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPROW_OWNER_MODULE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPROW_OWNER_MODULE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPROW_OWNER_MODULE {}
impl ::core::default::Default for MIB_TCPROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPROW_OWNER_PID {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
    pub dwOwningPid: u32,
}
impl ::core::marker::Copy for MIB_TCPROW_OWNER_PID {}
impl ::core::clone::Clone for MIB_TCPROW_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCPROW_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPROW_OWNER_PID").field("dwState", &self.dwState).field("dwLocalAddr", &self.dwLocalAddr).field("dwLocalPort", &self.dwLocalPort).field("dwRemoteAddr", &self.dwRemoteAddr).field("dwRemotePort", &self.dwRemotePort).field("dwOwningPid", &self.dwOwningPid).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPROW_OWNER_PID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPROW_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPROW_OWNER_PID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPROW_OWNER_PID {}
impl ::core::default::Default for MIB_TCPROW_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPROW_W2K {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
}
impl ::core::marker::Copy for MIB_TCPROW_W2K {}
impl ::core::clone::Clone for MIB_TCPROW_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCPROW_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPROW_W2K").field("dwState", &self.dwState).field("dwLocalAddr", &self.dwLocalAddr).field("dwLocalPort", &self.dwLocalPort).field("dwRemoteAddr", &self.dwRemoteAddr).field("dwRemotePort", &self.dwRemotePort).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPROW_W2K {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPROW_W2K {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPROW_W2K>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPROW_W2K {}
impl ::core::default::Default for MIB_TCPROW_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPSTATS2 {
    pub RtoAlgorithm: TCP_RTO_ALGORITHM,
    pub dwRtoMin: u32,
    pub dwRtoMax: u32,
    pub dwMaxConn: u32,
    pub dwActiveOpens: u32,
    pub dwPassiveOpens: u32,
    pub dwAttemptFails: u32,
    pub dwEstabResets: u32,
    pub dwCurrEstab: u32,
    pub dw64InSegs: u64,
    pub dw64OutSegs: u64,
    pub dwRetransSegs: u32,
    pub dwInErrs: u32,
    pub dwOutRsts: u32,
    pub dwNumConns: u32,
}
impl ::core::marker::Copy for MIB_TCPSTATS2 {}
impl ::core::clone::Clone for MIB_TCPSTATS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCPSTATS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPSTATS2")
            .field("RtoAlgorithm", &self.RtoAlgorithm)
            .field("dwRtoMin", &self.dwRtoMin)
            .field("dwRtoMax", &self.dwRtoMax)
            .field("dwMaxConn", &self.dwMaxConn)
            .field("dwActiveOpens", &self.dwActiveOpens)
            .field("dwPassiveOpens", &self.dwPassiveOpens)
            .field("dwAttemptFails", &self.dwAttemptFails)
            .field("dwEstabResets", &self.dwEstabResets)
            .field("dwCurrEstab", &self.dwCurrEstab)
            .field("dw64InSegs", &self.dw64InSegs)
            .field("dw64OutSegs", &self.dw64OutSegs)
            .field("dwRetransSegs", &self.dwRetransSegs)
            .field("dwInErrs", &self.dwInErrs)
            .field("dwOutRsts", &self.dwOutRsts)
            .field("dwNumConns", &self.dwNumConns)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPSTATS2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPSTATS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPSTATS2>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPSTATS2 {}
impl ::core::default::Default for MIB_TCPSTATS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPSTATS_LH {
    pub Anonymous: MIB_TCPSTATS_LH_0,
    pub dwRtoMin: u32,
    pub dwRtoMax: u32,
    pub dwMaxConn: u32,
    pub dwActiveOpens: u32,
    pub dwPassiveOpens: u32,
    pub dwAttemptFails: u32,
    pub dwEstabResets: u32,
    pub dwCurrEstab: u32,
    pub dwInSegs: u32,
    pub dwOutSegs: u32,
    pub dwRetransSegs: u32,
    pub dwInErrs: u32,
    pub dwOutRsts: u32,
    pub dwNumConns: u32,
}
impl ::core::marker::Copy for MIB_TCPSTATS_LH {}
impl ::core::clone::Clone for MIB_TCPSTATS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPSTATS_LH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPSTATS_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPSTATS_LH>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPSTATS_LH {}
impl ::core::default::Default for MIB_TCPSTATS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_TCPSTATS_LH_0 {
    pub dwRtoAlgorithm: u32,
    pub RtoAlgorithm: TCP_RTO_ALGORITHM,
}
impl ::core::marker::Copy for MIB_TCPSTATS_LH_0 {}
impl ::core::clone::Clone for MIB_TCPSTATS_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPSTATS_LH_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPSTATS_LH_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPSTATS_LH_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPSTATS_LH_0 {}
impl ::core::default::Default for MIB_TCPSTATS_LH_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPSTATS_W2K {
    pub dwRtoAlgorithm: u32,
    pub dwRtoMin: u32,
    pub dwRtoMax: u32,
    pub dwMaxConn: u32,
    pub dwActiveOpens: u32,
    pub dwPassiveOpens: u32,
    pub dwAttemptFails: u32,
    pub dwEstabResets: u32,
    pub dwCurrEstab: u32,
    pub dwInSegs: u32,
    pub dwOutSegs: u32,
    pub dwRetransSegs: u32,
    pub dwInErrs: u32,
    pub dwOutRsts: u32,
    pub dwNumConns: u32,
}
impl ::core::marker::Copy for MIB_TCPSTATS_W2K {}
impl ::core::clone::Clone for MIB_TCPSTATS_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCPSTATS_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPSTATS_W2K")
            .field("dwRtoAlgorithm", &self.dwRtoAlgorithm)
            .field("dwRtoMin", &self.dwRtoMin)
            .field("dwRtoMax", &self.dwRtoMax)
            .field("dwMaxConn", &self.dwMaxConn)
            .field("dwActiveOpens", &self.dwActiveOpens)
            .field("dwPassiveOpens", &self.dwPassiveOpens)
            .field("dwAttemptFails", &self.dwAttemptFails)
            .field("dwEstabResets", &self.dwEstabResets)
            .field("dwCurrEstab", &self.dwCurrEstab)
            .field("dwInSegs", &self.dwInSegs)
            .field("dwOutSegs", &self.dwOutSegs)
            .field("dwRetransSegs", &self.dwRetransSegs)
            .field("dwInErrs", &self.dwInErrs)
            .field("dwOutRsts", &self.dwOutRsts)
            .field("dwNumConns", &self.dwNumConns)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPSTATS_W2K {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPSTATS_W2K {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPSTATS_W2K>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPSTATS_W2K {}
impl ::core::default::Default for MIB_TCPSTATS_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW_LH; 1],
}
impl ::core::marker::Copy for MIB_TCPTABLE {}
impl ::core::clone::Clone for MIB_TCPTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPTABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPTABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPTABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPTABLE {}
impl ::core::default::Default for MIB_TCPTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPTABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW2; 1],
}
impl ::core::marker::Copy for MIB_TCPTABLE2 {}
impl ::core::clone::Clone for MIB_TCPTABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCPTABLE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPTABLE2").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPTABLE2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPTABLE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPTABLE2>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPTABLE2 {}
impl ::core::default::Default for MIB_TCPTABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPTABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW_OWNER_MODULE; 1],
}
impl ::core::marker::Copy for MIB_TCPTABLE_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_TCPTABLE_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCPTABLE_OWNER_MODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPTABLE_OWNER_MODULE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPTABLE_OWNER_MODULE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPTABLE_OWNER_MODULE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPTABLE_OWNER_MODULE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPTABLE_OWNER_MODULE {}
impl ::core::default::Default for MIB_TCPTABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPTABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW_OWNER_PID; 1],
}
impl ::core::marker::Copy for MIB_TCPTABLE_OWNER_PID {}
impl ::core::clone::Clone for MIB_TCPTABLE_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_TCPTABLE_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPTABLE_OWNER_PID").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_TCPTABLE_OWNER_PID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_TCPTABLE_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_TCPTABLE_OWNER_PID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_TCPTABLE_OWNER_PID {}
impl ::core::default::Default for MIB_TCPTABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIB_TCP_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_CLOSED: MIB_TCP_STATE = MIB_TCP_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_LISTEN: MIB_TCP_STATE = MIB_TCP_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_SYN_SENT: MIB_TCP_STATE = MIB_TCP_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_SYN_RCVD: MIB_TCP_STATE = MIB_TCP_STATE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_ESTAB: MIB_TCP_STATE = MIB_TCP_STATE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_FIN_WAIT1: MIB_TCP_STATE = MIB_TCP_STATE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_FIN_WAIT2: MIB_TCP_STATE = MIB_TCP_STATE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_CLOSE_WAIT: MIB_TCP_STATE = MIB_TCP_STATE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_CLOSING: MIB_TCP_STATE = MIB_TCP_STATE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_LAST_ACK: MIB_TCP_STATE = MIB_TCP_STATE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_TIME_WAIT: MIB_TCP_STATE = MIB_TCP_STATE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_DELETE_TCB: MIB_TCP_STATE = MIB_TCP_STATE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_RESERVED: MIB_TCP_STATE = MIB_TCP_STATE(100i32);
impl ::core::marker::Copy for MIB_TCP_STATE {}
impl ::core::clone::Clone for MIB_TCP_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIB_TCP_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MIB_TCP_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MIB_TCP_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_TCP_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_UDP6ROW {
    pub dwLocalAddr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_UDP6ROW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_UDP6ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MIB_UDP6ROW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MIB_UDP6ROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDP6ROW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MIB_UDP6ROW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_UDP6ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6ROW2 {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDP6ROW2_0,
    pub OwningModuleInfo: [u64; 16],
    pub ucRemoteAddr: [u8; 16],
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
}
impl ::core::marker::Copy for MIB_UDP6ROW2 {}
impl ::core::clone::Clone for MIB_UDP6ROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_UDP6ROW2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDP6ROW2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDP6ROW2>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDP6ROW2 {}
impl ::core::default::Default for MIB_UDP6ROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_UDP6ROW2_0 {
    pub Anonymous: MIB_UDP6ROW2_0_0,
    pub dwFlags: i32,
}
impl ::core::marker::Copy for MIB_UDP6ROW2_0 {}
impl ::core::clone::Clone for MIB_UDP6ROW2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_UDP6ROW2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDP6ROW2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDP6ROW2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDP6ROW2_0 {}
impl ::core::default::Default for MIB_UDP6ROW2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6ROW2_0_0 {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for MIB_UDP6ROW2_0_0 {}
impl ::core::clone::Clone for MIB_UDP6ROW2_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_UDP6ROW2_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDP6ROW2_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_UDP6ROW2_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDP6ROW2_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDP6ROW2_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDP6ROW2_0_0 {}
impl ::core::default::Default for MIB_UDP6ROW2_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6ROW_OWNER_MODULE {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDP6ROW_OWNER_MODULE_0,
    pub OwningModuleInfo: [u64; 16],
}
impl ::core::marker::Copy for MIB_UDP6ROW_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_UDP6ROW_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_UDP6ROW_OWNER_MODULE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDP6ROW_OWNER_MODULE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDP6ROW_OWNER_MODULE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDP6ROW_OWNER_MODULE {}
impl ::core::default::Default for MIB_UDP6ROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_UDP6ROW_OWNER_MODULE_0 {
    pub Anonymous: MIB_UDP6ROW_OWNER_MODULE_0_0,
    pub dwFlags: i32,
}
impl ::core::marker::Copy for MIB_UDP6ROW_OWNER_MODULE_0 {}
impl ::core::clone::Clone for MIB_UDP6ROW_OWNER_MODULE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_UDP6ROW_OWNER_MODULE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDP6ROW_OWNER_MODULE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDP6ROW_OWNER_MODULE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDP6ROW_OWNER_MODULE_0 {}
impl ::core::default::Default for MIB_UDP6ROW_OWNER_MODULE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6ROW_OWNER_MODULE_0_0 {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for MIB_UDP6ROW_OWNER_MODULE_0_0 {}
impl ::core::clone::Clone for MIB_UDP6ROW_OWNER_MODULE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_UDP6ROW_OWNER_MODULE_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDP6ROW_OWNER_MODULE_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_UDP6ROW_OWNER_MODULE_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDP6ROW_OWNER_MODULE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDP6ROW_OWNER_MODULE_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDP6ROW_OWNER_MODULE_0_0 {}
impl ::core::default::Default for MIB_UDP6ROW_OWNER_MODULE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6ROW_OWNER_PID {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
}
impl ::core::marker::Copy for MIB_UDP6ROW_OWNER_PID {}
impl ::core::clone::Clone for MIB_UDP6ROW_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_UDP6ROW_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDP6ROW_OWNER_PID").field("ucLocalAddr", &self.ucLocalAddr).field("dwLocalScopeId", &self.dwLocalScopeId).field("dwLocalPort", &self.dwLocalPort).field("dwOwningPid", &self.dwOwningPid).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_UDP6ROW_OWNER_PID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDP6ROW_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDP6ROW_OWNER_PID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDP6ROW_OWNER_PID {}
impl ::core::default::Default for MIB_UDP6ROW_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_UDP6TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_UDP6TABLE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_UDP6TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MIB_UDP6TABLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MIB_UDP6TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDP6TABLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MIB_UDP6TABLE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_UDP6TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6TABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW2; 1],
}
impl ::core::marker::Copy for MIB_UDP6TABLE2 {}
impl ::core::clone::Clone for MIB_UDP6TABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_UDP6TABLE2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDP6TABLE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDP6TABLE2>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDP6TABLE2 {}
impl ::core::default::Default for MIB_UDP6TABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6TABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW_OWNER_MODULE; 1],
}
impl ::core::marker::Copy for MIB_UDP6TABLE_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_UDP6TABLE_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_UDP6TABLE_OWNER_MODULE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDP6TABLE_OWNER_MODULE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDP6TABLE_OWNER_MODULE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDP6TABLE_OWNER_MODULE {}
impl ::core::default::Default for MIB_UDP6TABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6TABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW_OWNER_PID; 1],
}
impl ::core::marker::Copy for MIB_UDP6TABLE_OWNER_PID {}
impl ::core::clone::Clone for MIB_UDP6TABLE_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_UDP6TABLE_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDP6TABLE_OWNER_PID").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_UDP6TABLE_OWNER_PID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDP6TABLE_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDP6TABLE_OWNER_PID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDP6TABLE_OWNER_PID {}
impl ::core::default::Default for MIB_UDP6TABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPROW {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
}
impl ::core::marker::Copy for MIB_UDPROW {}
impl ::core::clone::Clone for MIB_UDPROW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_UDPROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPROW").field("dwLocalAddr", &self.dwLocalAddr).field("dwLocalPort", &self.dwLocalPort).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPROW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPROW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPROW {}
impl ::core::default::Default for MIB_UDPROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPROW2 {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDPROW2_0,
    pub OwningModuleInfo: [u64; 16],
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
}
impl ::core::marker::Copy for MIB_UDPROW2 {}
impl ::core::clone::Clone for MIB_UDPROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPROW2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPROW2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPROW2>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPROW2 {}
impl ::core::default::Default for MIB_UDPROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_UDPROW2_0 {
    pub Anonymous: MIB_UDPROW2_0_0,
    pub dwFlags: i32,
}
impl ::core::marker::Copy for MIB_UDPROW2_0 {}
impl ::core::clone::Clone for MIB_UDPROW2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPROW2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPROW2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPROW2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPROW2_0 {}
impl ::core::default::Default for MIB_UDPROW2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPROW2_0_0 {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for MIB_UDPROW2_0_0 {}
impl ::core::clone::Clone for MIB_UDPROW2_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_UDPROW2_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPROW2_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPROW2_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPROW2_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPROW2_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPROW2_0_0 {}
impl ::core::default::Default for MIB_UDPROW2_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPROW_OWNER_MODULE {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDPROW_OWNER_MODULE_0,
    pub OwningModuleInfo: [u64; 16],
}
impl ::core::marker::Copy for MIB_UDPROW_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_UDPROW_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPROW_OWNER_MODULE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPROW_OWNER_MODULE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPROW_OWNER_MODULE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPROW_OWNER_MODULE {}
impl ::core::default::Default for MIB_UDPROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_UDPROW_OWNER_MODULE_0 {
    pub Anonymous: MIB_UDPROW_OWNER_MODULE_0_0,
    pub dwFlags: i32,
}
impl ::core::marker::Copy for MIB_UDPROW_OWNER_MODULE_0 {}
impl ::core::clone::Clone for MIB_UDPROW_OWNER_MODULE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPROW_OWNER_MODULE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPROW_OWNER_MODULE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPROW_OWNER_MODULE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPROW_OWNER_MODULE_0 {}
impl ::core::default::Default for MIB_UDPROW_OWNER_MODULE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPROW_OWNER_MODULE_0_0 {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for MIB_UDPROW_OWNER_MODULE_0_0 {}
impl ::core::clone::Clone for MIB_UDPROW_OWNER_MODULE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_UDPROW_OWNER_MODULE_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPROW_OWNER_MODULE_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPROW_OWNER_MODULE_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPROW_OWNER_MODULE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPROW_OWNER_MODULE_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPROW_OWNER_MODULE_0_0 {}
impl ::core::default::Default for MIB_UDPROW_OWNER_MODULE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPROW_OWNER_PID {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
}
impl ::core::marker::Copy for MIB_UDPROW_OWNER_PID {}
impl ::core::clone::Clone for MIB_UDPROW_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_UDPROW_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPROW_OWNER_PID").field("dwLocalAddr", &self.dwLocalAddr).field("dwLocalPort", &self.dwLocalPort).field("dwOwningPid", &self.dwOwningPid).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPROW_OWNER_PID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPROW_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPROW_OWNER_PID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPROW_OWNER_PID {}
impl ::core::default::Default for MIB_UDPROW_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPSTATS {
    pub dwInDatagrams: u32,
    pub dwNoPorts: u32,
    pub dwInErrors: u32,
    pub dwOutDatagrams: u32,
    pub dwNumAddrs: u32,
}
impl ::core::marker::Copy for MIB_UDPSTATS {}
impl ::core::clone::Clone for MIB_UDPSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_UDPSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPSTATS").field("dwInDatagrams", &self.dwInDatagrams).field("dwNoPorts", &self.dwNoPorts).field("dwInErrors", &self.dwInErrors).field("dwOutDatagrams", &self.dwOutDatagrams).field("dwNumAddrs", &self.dwNumAddrs).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPSTATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPSTATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPSTATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPSTATS {}
impl ::core::default::Default for MIB_UDPSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPSTATS2 {
    pub dw64InDatagrams: u64,
    pub dwNoPorts: u32,
    pub dwInErrors: u32,
    pub dw64OutDatagrams: u64,
    pub dwNumAddrs: u32,
}
impl ::core::marker::Copy for MIB_UDPSTATS2 {}
impl ::core::clone::Clone for MIB_UDPSTATS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_UDPSTATS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPSTATS2").field("dw64InDatagrams", &self.dw64InDatagrams).field("dwNoPorts", &self.dwNoPorts).field("dwInErrors", &self.dwInErrors).field("dw64OutDatagrams", &self.dw64OutDatagrams).field("dwNumAddrs", &self.dwNumAddrs).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPSTATS2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPSTATS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPSTATS2>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPSTATS2 {}
impl ::core::default::Default for MIB_UDPSTATS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW; 1],
}
impl ::core::marker::Copy for MIB_UDPTABLE {}
impl ::core::clone::Clone for MIB_UDPTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_UDPTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPTABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPTABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPTABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPTABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPTABLE {}
impl ::core::default::Default for MIB_UDPTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPTABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW2; 1],
}
impl ::core::marker::Copy for MIB_UDPTABLE2 {}
impl ::core::clone::Clone for MIB_UDPTABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPTABLE2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPTABLE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPTABLE2>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPTABLE2 {}
impl ::core::default::Default for MIB_UDPTABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPTABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW_OWNER_MODULE; 1],
}
impl ::core::marker::Copy for MIB_UDPTABLE_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_UDPTABLE_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPTABLE_OWNER_MODULE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPTABLE_OWNER_MODULE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPTABLE_OWNER_MODULE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPTABLE_OWNER_MODULE {}
impl ::core::default::Default for MIB_UDPTABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPTABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW_OWNER_PID; 1],
}
impl ::core::marker::Copy for MIB_UDPTABLE_OWNER_PID {}
impl ::core::clone::Clone for MIB_UDPTABLE_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIB_UDPTABLE_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPTABLE_OWNER_PID").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
unsafe impl ::windows::core::Abi for MIB_UDPTABLE_OWNER_PID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIB_UDPTABLE_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UDPTABLE_OWNER_PID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIB_UDPTABLE_OWNER_PID {}
impl ::core::default::Default for MIB_UDPTABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_UNICASTIPADDRESS_ROW {
    pub Address: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceLuid: NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub PrefixOrigin: super::super::Networking::WinSock::NL_PREFIX_ORIGIN,
    pub SuffixOrigin: super::super::Networking::WinSock::NL_SUFFIX_ORIGIN,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub OnLinkPrefixLength: u8,
    pub SkipAsSource: super::super::Foundation::BOOLEAN,
    pub DadState: super::super::Networking::WinSock::NL_DAD_STATE,
    pub ScopeId: super::super::Networking::WinSock::SCOPE_ID,
    pub CreationTimeStamp: i64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_UNICASTIPADDRESS_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_UNICASTIPADDRESS_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_UNICASTIPADDRESS_ROW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_UNICASTIPADDRESS_ROW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UNICASTIPADDRESS_ROW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_UNICASTIPADDRESS_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_UNICASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_UNICASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_UNICASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_UNICASTIPADDRESS_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_UNICASTIPADDRESS_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MIB_UNICASTIPADDRESS_TABLE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_UNICASTIPADDRESS_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIB_UNICASTIPADDRESS_TABLE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_UNICASTIPADDRESS_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_UNICASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_USE_CURRENT_FORWARDING: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_USE_CURRENT_TTL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIN_IF_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIXED_NODETYPE: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NDIS_INTERFACE_INFORMATION {
    pub ifOperStatus: NET_IF_OPER_STATUS,
    pub ifOperStatusFlags: u32,
    pub MediaConnectState: NET_IF_MEDIA_CONNECT_STATE,
    pub MediaDuplexState: NET_IF_MEDIA_DUPLEX_STATE,
    pub ifMtu: u32,
    pub ifPromiscuousMode: super::super::Foundation::BOOLEAN,
    pub ifDeviceWakeUpEnable: super::super::Foundation::BOOLEAN,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub ifLastChange: u64,
    pub ifCounterDiscontinuityTime: u64,
    pub ifInUnknownProtos: u64,
    pub ifInDiscards: u64,
    pub ifInErrors: u64,
    pub ifHCInOctets: u64,
    pub ifHCInUcastPkts: u64,
    pub ifHCInMulticastPkts: u64,
    pub ifHCInBroadcastPkts: u64,
    pub ifHCOutOctets: u64,
    pub ifHCOutUcastPkts: u64,
    pub ifHCOutMulticastPkts: u64,
    pub ifHCOutBroadcastPkts: u64,
    pub ifOutErrors: u64,
    pub ifOutDiscards: u64,
    pub ifHCInUcastOctets: u64,
    pub ifHCInMulticastOctets: u64,
    pub ifHCInBroadcastOctets: u64,
    pub ifHCOutUcastOctets: u64,
    pub ifHCOutMulticastOctets: u64,
    pub ifHCOutBroadcastOctets: u64,
    pub CompartmentId: u32,
    pub SupportedStatistics: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NDIS_INTERFACE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NDIS_INTERFACE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NDIS_INTERFACE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_INTERFACE_INFORMATION")
            .field("ifOperStatus", &self.ifOperStatus)
            .field("ifOperStatusFlags", &self.ifOperStatusFlags)
            .field("MediaConnectState", &self.MediaConnectState)
            .field("MediaDuplexState", &self.MediaDuplexState)
            .field("ifMtu", &self.ifMtu)
            .field("ifPromiscuousMode", &self.ifPromiscuousMode)
            .field("ifDeviceWakeUpEnable", &self.ifDeviceWakeUpEnable)
            .field("XmitLinkSpeed", &self.XmitLinkSpeed)
            .field("RcvLinkSpeed", &self.RcvLinkSpeed)
            .field("ifLastChange", &self.ifLastChange)
            .field("ifCounterDiscontinuityTime", &self.ifCounterDiscontinuityTime)
            .field("ifInUnknownProtos", &self.ifInUnknownProtos)
            .field("ifInDiscards", &self.ifInDiscards)
            .field("ifInErrors", &self.ifInErrors)
            .field("ifHCInOctets", &self.ifHCInOctets)
            .field("ifHCInUcastPkts", &self.ifHCInUcastPkts)
            .field("ifHCInMulticastPkts", &self.ifHCInMulticastPkts)
            .field("ifHCInBroadcastPkts", &self.ifHCInBroadcastPkts)
            .field("ifHCOutOctets", &self.ifHCOutOctets)
            .field("ifHCOutUcastPkts", &self.ifHCOutUcastPkts)
            .field("ifHCOutMulticastPkts", &self.ifHCOutMulticastPkts)
            .field("ifHCOutBroadcastPkts", &self.ifHCOutBroadcastPkts)
            .field("ifOutErrors", &self.ifOutErrors)
            .field("ifOutDiscards", &self.ifOutDiscards)
            .field("ifHCInUcastOctets", &self.ifHCInUcastOctets)
            .field("ifHCInMulticastOctets", &self.ifHCInMulticastOctets)
            .field("ifHCInBroadcastOctets", &self.ifHCInBroadcastOctets)
            .field("ifHCOutUcastOctets", &self.ifHCOutUcastOctets)
            .field("ifHCOutMulticastOctets", &self.ifHCOutMulticastOctets)
            .field("ifHCOutBroadcastOctets", &self.ifHCOutBroadcastOctets)
            .field("CompartmentId", &self.CompartmentId)
            .field("SupportedStatistics", &self.SupportedStatistics)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NDIS_INTERFACE_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NDIS_INTERFACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDIS_INTERFACE_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NDIS_INTERFACE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NDIS_INTERFACE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_ADDRESS_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_ADDRESS_FORMAT_UNSPECIFIED: NET_ADDRESS_FORMAT = NET_ADDRESS_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_ADDRESS_DNS_NAME: NET_ADDRESS_FORMAT = NET_ADDRESS_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_ADDRESS_IPV4: NET_ADDRESS_FORMAT = NET_ADDRESS_FORMAT(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_ADDRESS_IPV6: NET_ADDRESS_FORMAT = NET_ADDRESS_FORMAT(3i32);
impl ::core::marker::Copy for NET_ADDRESS_FORMAT {}
impl ::core::clone::Clone for NET_ADDRESS_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_ADDRESS_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NET_ADDRESS_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_ADDRESS_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_ADDRESS_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IFLUID_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_IF_ACCESS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ACCESS_LOOPBACK: NET_IF_ACCESS_TYPE = NET_IF_ACCESS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ACCESS_BROADCAST: NET_IF_ACCESS_TYPE = NET_IF_ACCESS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ACCESS_POINT_TO_POINT: NET_IF_ACCESS_TYPE = NET_IF_ACCESS_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ACCESS_POINT_TO_MULTI_POINT: NET_IF_ACCESS_TYPE = NET_IF_ACCESS_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ACCESS_MAXIMUM: NET_IF_ACCESS_TYPE = NET_IF_ACCESS_TYPE(5i32);
impl ::core::marker::Copy for NET_IF_ACCESS_TYPE {}
impl ::core::clone::Clone for NET_IF_ACCESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_IF_ACCESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NET_IF_ACCESS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_IF_ACCESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_ACCESS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_IF_ADMIN_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ADMIN_STATUS_UP: NET_IF_ADMIN_STATUS = NET_IF_ADMIN_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ADMIN_STATUS_DOWN: NET_IF_ADMIN_STATUS = NET_IF_ADMIN_STATUS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ADMIN_STATUS_TESTING: NET_IF_ADMIN_STATUS = NET_IF_ADMIN_STATUS(3i32);
impl ::core::marker::Copy for NET_IF_ADMIN_STATUS {}
impl ::core::clone::Clone for NET_IF_ADMIN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_IF_ADMIN_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NET_IF_ADMIN_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_IF_ADMIN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_ADMIN_STATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct NET_IF_ALIAS_LH {
    pub ifAliasLength: u16,
    pub ifAliasOffset: u16,
}
impl ::core::marker::Copy for NET_IF_ALIAS_LH {}
impl ::core::clone::Clone for NET_IF_ALIAS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NET_IF_ALIAS_LH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_IF_ALIAS_LH").field("ifAliasLength", &self.ifAliasLength).field("ifAliasOffset", &self.ifAliasOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for NET_IF_ALIAS_LH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NET_IF_ALIAS_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_IF_ALIAS_LH>()) == 0 }
    }
}
impl ::core::cmp::Eq for NET_IF_ALIAS_LH {}
impl ::core::default::Default for NET_IF_ALIAS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_IF_CONNECTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_CONNECTION_DEDICATED: NET_IF_CONNECTION_TYPE = NET_IF_CONNECTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_CONNECTION_PASSIVE: NET_IF_CONNECTION_TYPE = NET_IF_CONNECTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_CONNECTION_DEMAND: NET_IF_CONNECTION_TYPE = NET_IF_CONNECTION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_CONNECTION_MAXIMUM: NET_IF_CONNECTION_TYPE = NET_IF_CONNECTION_TYPE(4i32);
impl ::core::marker::Copy for NET_IF_CONNECTION_TYPE {}
impl ::core::clone::Clone for NET_IF_CONNECTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_IF_CONNECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NET_IF_CONNECTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_IF_CONNECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_CONNECTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_IF_DIRECTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_DIRECTION_SENDRECEIVE: NET_IF_DIRECTION_TYPE = NET_IF_DIRECTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_DIRECTION_SENDONLY: NET_IF_DIRECTION_TYPE = NET_IF_DIRECTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_DIRECTION_RECEIVEONLY: NET_IF_DIRECTION_TYPE = NET_IF_DIRECTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_DIRECTION_MAXIMUM: NET_IF_DIRECTION_TYPE = NET_IF_DIRECTION_TYPE(3i32);
impl ::core::marker::Copy for NET_IF_DIRECTION_TYPE {}
impl ::core::clone::Clone for NET_IF_DIRECTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_IF_DIRECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NET_IF_DIRECTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_IF_DIRECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_DIRECTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_IF_MEDIA_CONNECT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MediaConnectStateUnknown: NET_IF_MEDIA_CONNECT_STATE = NET_IF_MEDIA_CONNECT_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MediaConnectStateConnected: NET_IF_MEDIA_CONNECT_STATE = NET_IF_MEDIA_CONNECT_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MediaConnectStateDisconnected: NET_IF_MEDIA_CONNECT_STATE = NET_IF_MEDIA_CONNECT_STATE(2i32);
impl ::core::marker::Copy for NET_IF_MEDIA_CONNECT_STATE {}
impl ::core::clone::Clone for NET_IF_MEDIA_CONNECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_IF_MEDIA_CONNECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NET_IF_MEDIA_CONNECT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_IF_MEDIA_CONNECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_MEDIA_CONNECT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_IF_MEDIA_DUPLEX_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MediaDuplexStateUnknown: NET_IF_MEDIA_DUPLEX_STATE = NET_IF_MEDIA_DUPLEX_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MediaDuplexStateHalf: NET_IF_MEDIA_DUPLEX_STATE = NET_IF_MEDIA_DUPLEX_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MediaDuplexStateFull: NET_IF_MEDIA_DUPLEX_STATE = NET_IF_MEDIA_DUPLEX_STATE(2i32);
impl ::core::marker::Copy for NET_IF_MEDIA_DUPLEX_STATE {}
impl ::core::clone::Clone for NET_IF_MEDIA_DUPLEX_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_IF_MEDIA_DUPLEX_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NET_IF_MEDIA_DUPLEX_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_IF_MEDIA_DUPLEX_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_MEDIA_DUPLEX_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OID_COMPARTMENT_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OID_IF_ALIAS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OID_IF_ENTRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OID_NETWORK_GUID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_IF_OPER_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_UP: NET_IF_OPER_STATUS = NET_IF_OPER_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_DOWN: NET_IF_OPER_STATUS = NET_IF_OPER_STATUS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_TESTING: NET_IF_OPER_STATUS = NET_IF_OPER_STATUS(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_UNKNOWN: NET_IF_OPER_STATUS = NET_IF_OPER_STATUS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_DORMANT: NET_IF_OPER_STATUS = NET_IF_OPER_STATUS(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_NOT_PRESENT: NET_IF_OPER_STATUS = NET_IF_OPER_STATUS(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_LOWER_LAYER_DOWN: NET_IF_OPER_STATUS = NET_IF_OPER_STATUS(7i32);
impl ::core::marker::Copy for NET_IF_OPER_STATUS {}
impl ::core::clone::Clone for NET_IF_OPER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_IF_OPER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NET_IF_OPER_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_IF_OPER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_OPER_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_DORMANT_LOW_POWER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_DORMANT_PAUSED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_DOWN_NOT_AUTHENTICATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_DOWN_NOT_MEDIA_CONNECTED: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct NET_IF_RCV_ADDRESS_LH {
    pub ifRcvAddressType: NET_IF_RCV_ADDRESS_TYPE,
    pub ifRcvAddressLength: u16,
    pub ifRcvAddressOffset: u16,
}
impl ::core::marker::Copy for NET_IF_RCV_ADDRESS_LH {}
impl ::core::clone::Clone for NET_IF_RCV_ADDRESS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NET_IF_RCV_ADDRESS_LH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_IF_RCV_ADDRESS_LH").field("ifRcvAddressType", &self.ifRcvAddressType).field("ifRcvAddressLength", &self.ifRcvAddressLength).field("ifRcvAddressOffset", &self.ifRcvAddressOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for NET_IF_RCV_ADDRESS_LH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NET_IF_RCV_ADDRESS_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_IF_RCV_ADDRESS_LH>()) == 0 }
    }
}
impl ::core::cmp::Eq for NET_IF_RCV_ADDRESS_LH {}
impl ::core::default::Default for NET_IF_RCV_ADDRESS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NET_IF_RCV_ADDRESS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_RCV_ADDRESS_TYPE_OTHER: NET_IF_RCV_ADDRESS_TYPE = NET_IF_RCV_ADDRESS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_RCV_ADDRESS_TYPE_VOLATILE: NET_IF_RCV_ADDRESS_TYPE = NET_IF_RCV_ADDRESS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_RCV_ADDRESS_TYPE_NON_VOLATILE: NET_IF_RCV_ADDRESS_TYPE = NET_IF_RCV_ADDRESS_TYPE(3i32);
impl ::core::marker::Copy for NET_IF_RCV_ADDRESS_TYPE {}
impl ::core::clone::Clone for NET_IF_RCV_ADDRESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_IF_RCV_ADDRESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NET_IF_RCV_ADDRESS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_IF_RCV_ADDRESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_RCV_ADDRESS_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union NET_LUID_LH {
    pub Value: u64,
    pub Info: NET_LUID_LH_0,
}
impl ::core::marker::Copy for NET_LUID_LH {}
impl ::core::clone::Clone for NET_LUID_LH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NET_LUID_LH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NET_LUID_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_LUID_LH>()) == 0 }
    }
}
impl ::core::cmp::Eq for NET_LUID_LH {}
impl ::core::default::Default for NET_LUID_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct NET_LUID_LH_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for NET_LUID_LH_0 {}
impl ::core::clone::Clone for NET_LUID_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NET_LUID_LH_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_LUID_LH_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NET_LUID_LH_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NET_LUID_LH_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_LUID_LH_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for NET_LUID_LH_0 {}
impl ::core::default::Default for NET_LUID_LH_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct NET_PHYSICAL_LOCATION_LH {
    pub BusNumber: u32,
    pub SlotNumber: u32,
    pub FunctionNumber: u32,
}
impl ::core::marker::Copy for NET_PHYSICAL_LOCATION_LH {}
impl ::core::clone::Clone for NET_PHYSICAL_LOCATION_LH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NET_PHYSICAL_LOCATION_LH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_PHYSICAL_LOCATION_LH").field("BusNumber", &self.BusNumber).field("SlotNumber", &self.SlotNumber).field("FunctionNumber", &self.FunctionNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for NET_PHYSICAL_LOCATION_LH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NET_PHYSICAL_LOCATION_LH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_PHYSICAL_LOCATION_LH>()) == 0 }
    }
}
impl ::core::cmp::Eq for NET_PHYSICAL_LOCATION_LH {}
impl ::core::default::Default for NET_PHYSICAL_LOCATION_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_SITEID_MAXSYSTEM: u32 = 268435455u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_SITEID_MAXUSER: u32 = 134217727u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_SITEID_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV4_ADDRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV4_NETWORK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV4_SERVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV6_ADDRESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV6_ADDRESS_NO_SCOPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV6_NETWORK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV6_SERVICE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV6_SERVICE_NO_SCOPE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_NAMED_ADDRESS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_NAMED_SERVICE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_FILTER_INTERFACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_HARDWARE_INTERFACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_ENDPOINT_INTERFACE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_ISCSI_INTERFACE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_RESERVED1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_RESERVED2: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_RESERVED3: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_RESERVED4: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_WDM_INTERFACE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NUMBER_OF_EXPORTED_VARIABLES: u32 = 39u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NhpAllocateAndGetInterfaceInfoFromStack<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pptable: *mut *mut ip_interface_name_info_w2ksp1, pdwcount: *mut u32, border: Param2, hheap: Param3, dwflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NhpAllocateAndGetInterfaceInfoFromStack(pptable: *mut *mut ip_interface_name_info_w2ksp1, pdwcount: *mut u32, border: super::super::Foundation::BOOL, hheap: super::super::Foundation::HANDLE, dwflags: u32) -> u32;
        }
        ::core::mem::transmute(NhpAllocateAndGetInterfaceInfoFromStack(::core::mem::transmute(pptable), ::core::mem::transmute(pdwcount), border.into_param().abi(), hheap.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn NotifyAddrChange(handle: *mut super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyAddrChange(handle: *mut super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(NotifyAddrChange(::core::mem::transmute(handle), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn NotifyIpInterfaceChange<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(family: u16, callback: PIPINTERFACE_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: Param3, notificationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyIpInterfaceChange(family: u16, callback: ::windows::core::RawPtr, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
        }
        NotifyIpInterfaceChange(::core::mem::transmute(family), ::core::mem::transmute(callback), ::core::mem::transmute(callercontext), initialnotification.into_param().abi(), ::core::mem::transmute(notificationhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn NotifyNetworkConnectivityHintChange<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(callback: PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: Param2, notificationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyNetworkConnectivityHintChange(callback: ::windows::core::RawPtr, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
        }
        NotifyNetworkConnectivityHintChange(::core::mem::transmute(callback), ::core::mem::transmute(callercontext), initialnotification.into_param().abi(), ::core::mem::transmute(notificationhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn NotifyRouteChange(handle: *mut super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyRouteChange(handle: *mut super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
        }
        ::core::mem::transmute(NotifyRouteChange(::core::mem::transmute(handle), ::core::mem::transmute(overlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn NotifyRouteChange2<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(addressfamily: u16, callback: PIPFORWARD_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: Param3, notificationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyRouteChange2(addressfamily: u16, callback: ::windows::core::RawPtr, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
        }
        NotifyRouteChange2(::core::mem::transmute(addressfamily), ::core::mem::transmute(callback), ::core::mem::transmute(callercontext), initialnotification.into_param().abi(), ::core::mem::transmute(notificationhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn NotifyStableUnicastIpAddressTable(family: u16, table: *mut *mut MIB_UNICASTIPADDRESS_TABLE, callercallback: PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK, callercontext: *const ::core::ffi::c_void, notificationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyStableUnicastIpAddressTable(family: u16, table: *mut *mut MIB_UNICASTIPADDRESS_TABLE, callercallback: ::windows::core::RawPtr, callercontext: *const ::core::ffi::c_void, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
        }
        NotifyStableUnicastIpAddressTable(::core::mem::transmute(family), ::core::mem::transmute(table), ::core::mem::transmute(callercallback), ::core::mem::transmute(callercontext), ::core::mem::transmute(notificationhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NotifyTeredoPortChange<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(callback: PTEREDO_PORT_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: Param2, notificationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyTeredoPortChange(callback: ::windows::core::RawPtr, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
        }
        NotifyTeredoPortChange(::core::mem::transmute(callback), ::core::mem::transmute(callercontext), initialnotification.into_param().abi(), ::core::mem::transmute(notificationhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn NotifyUnicastIpAddressChange<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(family: u16, callback: PUNICAST_IPADDRESS_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: Param3, notificationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyUnicastIpAddressChange(family: u16, callback: ::windows::core::RawPtr, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
        }
        NotifyUnicastIpAddressChange(::core::mem::transmute(family), ::core::mem::transmute(callback), ::core::mem::transmute(callercontext), initialnotification.into_param().abi(), ::core::mem::transmute(notificationhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PEER_TO_PEER_NODETYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PFADDRESSTYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PF_IPV4: PFADDRESSTYPE = PFADDRESSTYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PF_IPV6: PFADDRESSTYPE = PFADDRESSTYPE(1i32);
impl ::core::marker::Copy for PFADDRESSTYPE {}
impl ::core::clone::Clone for PFADDRESSTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PFADDRESSTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PFADDRESSTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PFADDRESSTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PFADDRESSTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PFERROR_BUFFER_TOO_SMALL: u32 = 23002u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PFERROR_NO_FILTERS_GIVEN: u32 = 23001u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PFERROR_NO_PF_INTERFACE: u32 = 23000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PFFORWARD_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PF_ACTION_FORWARD: PFFORWARD_ACTION = PFFORWARD_ACTION(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PF_ACTION_DROP: PFFORWARD_ACTION = PFFORWARD_ACTION(1i32);
impl ::core::marker::Copy for PFFORWARD_ACTION {}
impl ::core::clone::Clone for PFFORWARD_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PFFORWARD_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PFFORWARD_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for PFFORWARD_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PFFORWARD_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PFFRAMETYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PFFT_FILTER: PFFRAMETYPE = PFFRAMETYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PFFT_FRAG: PFFRAMETYPE = PFFRAMETYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PFFT_SPOOF: PFFRAMETYPE = PFFRAMETYPE(3i32);
impl ::core::marker::Copy for PFFRAMETYPE {}
impl ::core::clone::Clone for PFFRAMETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PFFRAMETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PFFRAMETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PFFRAMETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PFFRAMETYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct PFLOGFRAME {
    pub Timestamp: i64,
    pub pfeTypeOfFrame: PFFRAMETYPE,
    pub dwTotalSizeUsed: u32,
    pub dwFilterRule: u32,
    pub wSizeOfAdditionalData: u16,
    pub wSizeOfIpHeader: u16,
    pub dwInterfaceName: u32,
    pub dwIPIndex: u32,
    pub bPacketData: [u8; 1],
}
impl ::core::marker::Copy for PFLOGFRAME {}
impl ::core::clone::Clone for PFLOGFRAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PFLOGFRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PFLOGFRAME").field("Timestamp", &self.Timestamp).field("pfeTypeOfFrame", &self.pfeTypeOfFrame).field("dwTotalSizeUsed", &self.dwTotalSizeUsed).field("dwFilterRule", &self.dwFilterRule).field("wSizeOfAdditionalData", &self.wSizeOfAdditionalData).field("wSizeOfIpHeader", &self.wSizeOfIpHeader).field("dwInterfaceName", &self.dwInterfaceName).field("dwIPIndex", &self.dwIPIndex).field("bPacketData", &self.bPacketData).finish()
    }
}
unsafe impl ::windows::core::Abi for PFLOGFRAME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PFLOGFRAME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PFLOGFRAME>()) == 0 }
    }
}
impl ::core::cmp::Eq for PFLOGFRAME {}
impl ::core::default::Default for PFLOGFRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct PF_FILTER_DESCRIPTOR {
    pub dwFilterFlags: u32,
    pub dwRule: u32,
    pub pfatType: PFADDRESSTYPE,
    pub SrcAddr: *mut u8,
    pub SrcMask: *mut u8,
    pub DstAddr: *mut u8,
    pub DstMask: *mut u8,
    pub dwProtocol: u32,
    pub fLateBound: u32,
    pub wSrcPort: u16,
    pub wDstPort: u16,
    pub wSrcPortHighRange: u16,
    pub wDstPortHighRange: u16,
}
impl ::core::marker::Copy for PF_FILTER_DESCRIPTOR {}
impl ::core::clone::Clone for PF_FILTER_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PF_FILTER_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PF_FILTER_DESCRIPTOR")
            .field("dwFilterFlags", &self.dwFilterFlags)
            .field("dwRule", &self.dwRule)
            .field("pfatType", &self.pfatType)
            .field("SrcAddr", &self.SrcAddr)
            .field("SrcMask", &self.SrcMask)
            .field("DstAddr", &self.DstAddr)
            .field("DstMask", &self.DstMask)
            .field("dwProtocol", &self.dwProtocol)
            .field("fLateBound", &self.fLateBound)
            .field("wSrcPort", &self.wSrcPort)
            .field("wDstPort", &self.wDstPort)
            .field("wSrcPortHighRange", &self.wSrcPortHighRange)
            .field("wDstPortHighRange", &self.wDstPortHighRange)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for PF_FILTER_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PF_FILTER_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PF_FILTER_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for PF_FILTER_DESCRIPTOR {}
impl ::core::default::Default for PF_FILTER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct PF_FILTER_STATS {
    pub dwNumPacketsFiltered: u32,
    pub info: PF_FILTER_DESCRIPTOR,
}
impl ::core::marker::Copy for PF_FILTER_STATS {}
impl ::core::clone::Clone for PF_FILTER_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PF_FILTER_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PF_FILTER_STATS").field("dwNumPacketsFiltered", &self.dwNumPacketsFiltered).field("info", &self.info).finish()
    }
}
unsafe impl ::windows::core::Abi for PF_FILTER_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PF_FILTER_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PF_FILTER_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PF_FILTER_STATS {}
impl ::core::default::Default for PF_FILTER_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct PF_INTERFACE_STATS {
    pub pvDriverContext: *mut ::core::ffi::c_void,
    pub dwFlags: u32,
    pub dwInDrops: u32,
    pub dwOutDrops: u32,
    pub eaInAction: PFFORWARD_ACTION,
    pub eaOutAction: PFFORWARD_ACTION,
    pub dwNumInFilters: u32,
    pub dwNumOutFilters: u32,
    pub dwFrag: u32,
    pub dwSpoof: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub liSYN: i64,
    pub liTotalLogged: i64,
    pub dwLostLogEntries: u32,
    pub FilterInfo: [PF_FILTER_STATS; 1],
}
impl ::core::marker::Copy for PF_INTERFACE_STATS {}
impl ::core::clone::Clone for PF_INTERFACE_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PF_INTERFACE_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PF_INTERFACE_STATS")
            .field("pvDriverContext", &self.pvDriverContext)
            .field("dwFlags", &self.dwFlags)
            .field("dwInDrops", &self.dwInDrops)
            .field("dwOutDrops", &self.dwOutDrops)
            .field("eaInAction", &self.eaInAction)
            .field("eaOutAction", &self.eaOutAction)
            .field("dwNumInFilters", &self.dwNumInFilters)
            .field("dwNumOutFilters", &self.dwNumOutFilters)
            .field("dwFrag", &self.dwFrag)
            .field("dwSpoof", &self.dwSpoof)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("liSYN", &self.liSYN)
            .field("liTotalLogged", &self.liTotalLogged)
            .field("dwLostLogEntries", &self.dwLostLogEntries)
            .field("FilterInfo", &self.FilterInfo)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for PF_INTERFACE_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PF_INTERFACE_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PF_INTERFACE_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PF_INTERFACE_STATS {}
impl ::core::default::Default for PF_INTERFACE_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct PF_LATEBIND_INFO {
    pub SrcAddr: *mut u8,
    pub DstAddr: *mut u8,
    pub Mask: *mut u8,
}
impl ::core::marker::Copy for PF_LATEBIND_INFO {}
impl ::core::clone::Clone for PF_LATEBIND_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PF_LATEBIND_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PF_LATEBIND_INFO").field("SrcAddr", &self.SrcAddr).field("DstAddr", &self.DstAddr).field("Mask", &self.Mask).finish()
    }
}
unsafe impl ::windows::core::Abi for PF_LATEBIND_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PF_LATEBIND_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PF_LATEBIND_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PF_LATEBIND_INFO {}
impl ::core::default::Default for PF_LATEBIND_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PIPFORWARD_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void, row: *const MIB_IPFORWARD_ROW2, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PIPINTERFACE_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void, row: *const MIB_IPINTERFACE_ROW, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void, connectivityhint: super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PROXY_ARP: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void, addresstable: *const MIB_UNICASTIPADDRESS_TABLE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type PTEREDO_PORT_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void, port: u16, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PUNICAST_IPADDRESS_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void, row: *const MIB_UNICASTIPADDRESS_ROW, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfAddFiltersToInterface(ih: *mut ::core::ffi::c_void, cinfilters: u32, pfiltin: *mut PF_FILTER_DESCRIPTOR, coutfilters: u32, pfiltout: *mut PF_FILTER_DESCRIPTOR, pfhandle: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfAddFiltersToInterface(ih: *mut ::core::ffi::c_void, cinfilters: u32, pfiltin: *mut PF_FILTER_DESCRIPTOR, coutfilters: u32, pfiltout: *mut PF_FILTER_DESCRIPTOR, pfhandle: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(PfAddFiltersToInterface(::core::mem::transmute(ih), ::core::mem::transmute(cinfilters), ::core::mem::transmute(pfiltin), ::core::mem::transmute(coutfilters), ::core::mem::transmute(pfiltout), ::core::mem::transmute(pfhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfAddGlobalFilterToInterface(pinterface: *mut ::core::ffi::c_void, gffilter: GLOBAL_FILTER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfAddGlobalFilterToInterface(pinterface: *mut ::core::ffi::c_void, gffilter: GLOBAL_FILTER) -> u32;
        }
        ::core::mem::transmute(PfAddGlobalFilterToInterface(::core::mem::transmute(pinterface), ::core::mem::transmute(gffilter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfBindInterfaceToIPAddress(pinterface: *mut ::core::ffi::c_void, pfattype: PFADDRESSTYPE, ipaddress: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfBindInterfaceToIPAddress(pinterface: *mut ::core::ffi::c_void, pfattype: PFADDRESSTYPE, ipaddress: *mut u8) -> u32;
        }
        ::core::mem::transmute(PfBindInterfaceToIPAddress(::core::mem::transmute(pinterface), ::core::mem::transmute(pfattype), ::core::mem::transmute(ipaddress)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfBindInterfaceToIndex(pinterface: *mut ::core::ffi::c_void, dwindex: u32, pfatlinktype: PFADDRESSTYPE, linkipaddress: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfBindInterfaceToIndex(pinterface: *mut ::core::ffi::c_void, dwindex: u32, pfatlinktype: PFADDRESSTYPE, linkipaddress: *mut u8) -> u32;
        }
        ::core::mem::transmute(PfBindInterfaceToIndex(::core::mem::transmute(pinterface), ::core::mem::transmute(dwindex), ::core::mem::transmute(pfatlinktype), ::core::mem::transmute(linkipaddress)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PfCreateInterface<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(dwname: u32, inaction: PFFORWARD_ACTION, outaction: PFFORWARD_ACTION, buselog: Param3, bmustbeunique: Param4, ppinterface: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfCreateInterface(dwname: u32, inaction: PFFORWARD_ACTION, outaction: PFFORWARD_ACTION, buselog: super::super::Foundation::BOOL, bmustbeunique: super::super::Foundation::BOOL, ppinterface: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(PfCreateInterface(::core::mem::transmute(dwname), ::core::mem::transmute(inaction), ::core::mem::transmute(outaction), buselog.into_param().abi(), bmustbeunique.into_param().abi(), ::core::mem::transmute(ppinterface)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfDeleteInterface(pinterface: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfDeleteInterface(pinterface: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(PfDeleteInterface(::core::mem::transmute(pinterface)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfDeleteLog() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfDeleteLog() -> u32;
        }
        ::core::mem::transmute(PfDeleteLog())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PfGetInterfaceStatistics<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pinterface: *mut ::core::ffi::c_void, ppfstats: *mut PF_INTERFACE_STATS, pdwbuffersize: *mut u32, fresetcounters: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfGetInterfaceStatistics(pinterface: *mut ::core::ffi::c_void, ppfstats: *mut PF_INTERFACE_STATS, pdwbuffersize: *mut u32, fresetcounters: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(PfGetInterfaceStatistics(::core::mem::transmute(pinterface), ::core::mem::transmute(ppfstats), ::core::mem::transmute(pdwbuffersize), fresetcounters.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PfMakeLog<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hevent: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfMakeLog(hevent: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(PfMakeLog(hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfRebindFilters(pinterface: *mut ::core::ffi::c_void, platebindinfo: *mut PF_LATEBIND_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfRebindFilters(pinterface: *mut ::core::ffi::c_void, platebindinfo: *mut PF_LATEBIND_INFO) -> u32;
        }
        ::core::mem::transmute(PfRebindFilters(::core::mem::transmute(pinterface), ::core::mem::transmute(platebindinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfRemoveFilterHandles(pinterface: *mut ::core::ffi::c_void, cfilters: u32, pvhandles: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfRemoveFilterHandles(pinterface: *mut ::core::ffi::c_void, cfilters: u32, pvhandles: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(PfRemoveFilterHandles(::core::mem::transmute(pinterface), ::core::mem::transmute(cfilters), ::core::mem::transmute(pvhandles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfRemoveFiltersFromInterface(ih: *mut ::core::ffi::c_void, cinfilters: u32, pfiltin: *mut PF_FILTER_DESCRIPTOR, coutfilters: u32, pfiltout: *mut PF_FILTER_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfRemoveFiltersFromInterface(ih: *mut ::core::ffi::c_void, cinfilters: u32, pfiltin: *mut PF_FILTER_DESCRIPTOR, coutfilters: u32, pfiltout: *mut PF_FILTER_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(PfRemoveFiltersFromInterface(::core::mem::transmute(ih), ::core::mem::transmute(cinfilters), ::core::mem::transmute(pfiltin), ::core::mem::transmute(coutfilters), ::core::mem::transmute(pfiltout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfRemoveGlobalFilterFromInterface(pinterface: *mut ::core::ffi::c_void, gffilter: GLOBAL_FILTER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfRemoveGlobalFilterFromInterface(pinterface: *mut ::core::ffi::c_void, gffilter: GLOBAL_FILTER) -> u32;
        }
        ::core::mem::transmute(PfRemoveGlobalFilterFromInterface(::core::mem::transmute(pinterface), ::core::mem::transmute(gffilter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfSetLogBuffer(pbbuffer: *mut u8, dwsize: u32, dwthreshold: u32, dwentries: u32, pdwloggedentries: *mut u32, pdwlostentries: *mut u32, pdwsizeused: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfSetLogBuffer(pbbuffer: *mut u8, dwsize: u32, dwthreshold: u32, dwentries: u32, pdwloggedentries: *mut u32, pdwlostentries: *mut u32, pdwsizeused: *mut u32) -> u32;
        }
        ::core::mem::transmute(PfSetLogBuffer(::core::mem::transmute(pbbuffer), ::core::mem::transmute(dwsize), ::core::mem::transmute(dwthreshold), ::core::mem::transmute(dwentries), ::core::mem::transmute(pdwloggedentries), ::core::mem::transmute(pdwlostentries), ::core::mem::transmute(pdwsizeused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfTestPacket(pininterface: *mut ::core::ffi::c_void, poutinterface: *mut ::core::ffi::c_void, cbytes: u32, pbpacket: *mut u8, ppaction: *mut PFFORWARD_ACTION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfTestPacket(pininterface: *mut ::core::ffi::c_void, poutinterface: *mut ::core::ffi::c_void, cbytes: u32, pbpacket: *mut u8, ppaction: *mut PFFORWARD_ACTION) -> u32;
        }
        ::core::mem::transmute(PfTestPacket(::core::mem::transmute(pininterface), ::core::mem::transmute(poutinterface), ::core::mem::transmute(cbytes), ::core::mem::transmute(pbpacket), ::core::mem::transmute(ppaction)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn PfUnBindInterface(pinterface: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PfUnBindInterface(pinterface: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(PfUnBindInterface(::core::mem::transmute(pinterface)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ROUTE_LONGER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ROUTE_MATCHING: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ROUTE_SHORTER: u32 = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ROUTE_STATE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn RegisterInterfaceTimestampConfigChange(callback: PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterInterfaceTimestampConfigChange(callback: ::windows::core::RawPtr, callercontext: *const ::core::ffi::c_void, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> u32;
        }
        ::core::mem::transmute(RegisterInterfaceTimestampConfigChange(::core::mem::transmute(callback), ::core::mem::transmute(callercontext), ::core::mem::transmute(notificationhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn ResolveIpNetEntry2(row: *mut MIB_IPNET_ROW2, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_INET) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResolveIpNetEntry2(row: *mut MIB_IPNET_ROW2, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_INET) -> super::super::Foundation::NTSTATUS;
        }
        ResolveIpNetEntry2(::core::mem::transmute(row), ::core::mem::transmute(sourceaddress)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn ResolveNeighbor(networkaddress: *const super::super::Networking::WinSock::SOCKADDR, physicaladdress: *mut ::core::ffi::c_void, physicaladdresslength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResolveNeighbor(networkaddress: *const super::super::Networking::WinSock::SOCKADDR, physicaladdress: *mut ::core::ffi::c_void, physicaladdresslength: *mut u32) -> u32;
        }
        ::core::mem::transmute(ResolveNeighbor(::core::mem::transmute(networkaddress), ::core::mem::transmute(physicaladdress), ::core::mem::transmute(physicaladdresslength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RestoreMediaSense(poverlapped: *const super::super::System::IO::OVERLAPPED, lpdwenablecount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RestoreMediaSense(poverlapped: *const super::super::System::IO::OVERLAPPED, lpdwenablecount: *mut u32) -> u32;
        }
        ::core::mem::transmute(RestoreMediaSense(::core::mem::transmute(poverlapped), ::core::mem::transmute(lpdwenablecount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn SendARP(destip: u32, srcip: u32, pmacaddr: *mut ::core::ffi::c_void, phyaddrlen: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SendARP(destip: u32, srcip: u32, pmacaddr: *mut ::core::ffi::c_void, phyaddrlen: *mut u32) -> u32;
        }
        ::core::mem::transmute(SendARP(::core::mem::transmute(destip), ::core::mem::transmute(srcip), ::core::mem::transmute(pmacaddr), ::core::mem::transmute(phyaddrlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCurrentThreadCompartmentId(compartmentid: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCurrentThreadCompartmentId(compartmentid: u32) -> super::super::Foundation::NTSTATUS;
        }
        SetCurrentThreadCompartmentId(::core::mem::transmute(compartmentid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCurrentThreadCompartmentScope(compartmentscope: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCurrentThreadCompartmentScope(compartmentscope: u32) -> super::super::Foundation::NTSTATUS;
        }
        SetCurrentThreadCompartmentScope(::core::mem::transmute(compartmentscope)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDnsSettings(settings: *const DNS_SETTINGS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDnsSettings(settings: *const DNS_SETTINGS) -> super::super::Foundation::NTSTATUS;
        }
        SetDnsSettings(::core::mem::transmute(settings)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn SetIfEntry(pifrow: *const MIB_IFROW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIfEntry(pifrow: *const MIB_IFROW) -> u32;
        }
        ::core::mem::transmute(SetIfEntry(::core::mem::transmute(pifrow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetInterfaceDnsSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(interface: Param0, settings: *const DNS_INTERFACE_SETTINGS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetInterfaceDnsSettings(interface: ::windows::core::GUID, settings: *const DNS_INTERFACE_SETTINGS) -> super::super::Foundation::NTSTATUS;
        }
        SetInterfaceDnsSettings(interface.into_param().abi(), ::core::mem::transmute(settings)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn SetIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32;
        }
        ::core::mem::transmute(SetIpForwardEntry(::core::mem::transmute(proute)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn SetIpForwardEntry2(route: *const MIB_IPFORWARD_ROW2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIpForwardEntry2(route: *const MIB_IPFORWARD_ROW2) -> super::super::Foundation::NTSTATUS;
        }
        SetIpForwardEntry2(::core::mem::transmute(route)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn SetIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) -> super::super::Foundation::NTSTATUS;
        }
        SetIpInterfaceEntry(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn SetIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32;
        }
        ::core::mem::transmute(SetIpNetEntry(::core::mem::transmute(parpentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn SetIpNetEntry2(row: *const MIB_IPNET_ROW2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::super::Foundation::NTSTATUS;
        }
        SetIpNetEntry2(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn SetIpStatistics(pipstats: *const MIB_IPSTATS_LH) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIpStatistics(pipstats: *const MIB_IPSTATS_LH) -> u32;
        }
        ::core::mem::transmute(SetIpStatistics(::core::mem::transmute(pipstats)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn SetIpStatisticsEx(statistics: *const MIB_IPSTATS_LH, family: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIpStatisticsEx(statistics: *const MIB_IPSTATS_LH, family: u32) -> u32;
        }
        ::core::mem::transmute(SetIpStatisticsEx(::core::mem::transmute(statistics), ::core::mem::transmute(family)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn SetIpTTL(nttl: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIpTTL(nttl: u32) -> u32;
        }
        ::core::mem::transmute(SetIpTTL(::core::mem::transmute(nttl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetJobCompartmentId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(jobhandle: Param0, compartmentid: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetJobCompartmentId(jobhandle: super::super::Foundation::HANDLE, compartmentid: u32) -> super::super::Foundation::NTSTATUS;
        }
        SetJobCompartmentId(jobhandle.into_param().abi(), ::core::mem::transmute(compartmentid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetNetworkInformation<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(networkguid: *const ::windows::core::GUID, compartmentid: u32, networkname: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetNetworkInformation(networkguid: *const ::windows::core::GUID, compartmentid: u32, networkname: ::windows::core::PCWSTR) -> super::super::Foundation::NTSTATUS;
        }
        SetNetworkInformation(::core::mem::transmute(networkguid), ::core::mem::transmute(compartmentid), networkname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn SetPerTcp6ConnectionEStats(row: *const MIB_TCP6ROW, estatstype: TCP_ESTATS_TYPE, rw: *const u8, rwversion: u32, rwsize: u32, offset: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPerTcp6ConnectionEStats(row: *const MIB_TCP6ROW, estatstype: TCP_ESTATS_TYPE, rw: *const u8, rwversion: u32, rwsize: u32, offset: u32) -> u32;
        }
        ::core::mem::transmute(SetPerTcp6ConnectionEStats(::core::mem::transmute(row), ::core::mem::transmute(estatstype), ::core::mem::transmute(rw), ::core::mem::transmute(rwversion), ::core::mem::transmute(rwsize), ::core::mem::transmute(offset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn SetPerTcpConnectionEStats(row: *const MIB_TCPROW_LH, estatstype: TCP_ESTATS_TYPE, rw: *const u8, rwversion: u32, rwsize: u32, offset: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPerTcpConnectionEStats(row: *const MIB_TCPROW_LH, estatstype: TCP_ESTATS_TYPE, rw: *const u8, rwversion: u32, rwsize: u32, offset: u32) -> u32;
        }
        ::core::mem::transmute(SetPerTcpConnectionEStats(::core::mem::transmute(row), ::core::mem::transmute(estatstype), ::core::mem::transmute(rw), ::core::mem::transmute(rwversion), ::core::mem::transmute(rwsize), ::core::mem::transmute(offset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSessionCompartmentId(sessionid: u32, compartmentid: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSessionCompartmentId(sessionid: u32, compartmentid: u32) -> super::super::Foundation::NTSTATUS;
        }
        SetSessionCompartmentId(::core::mem::transmute(sessionid), ::core::mem::transmute(compartmentid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn SetTcpEntry(ptcprow: *const MIB_TCPROW_LH) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTcpEntry(ptcprow: *const MIB_TCPROW_LH) -> u32;
        }
        ::core::mem::transmute(SetTcpEntry(::core::mem::transmute(ptcprow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn SetUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
        }
        SetUnicastIpAddressEntry(::core::mem::transmute(row)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP6_STATS: u32 = 38u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCPIP_OWNER_MODULE_BASIC_INFO {
    pub pModuleName: ::windows::core::PWSTR,
    pub pModulePath: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for TCPIP_OWNER_MODULE_BASIC_INFO {}
impl ::core::clone::Clone for TCPIP_OWNER_MODULE_BASIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCPIP_OWNER_MODULE_BASIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCPIP_OWNER_MODULE_BASIC_INFO").field("pModuleName", &self.pModuleName).field("pModulePath", &self.pModulePath).finish()
    }
}
unsafe impl ::windows::core::Abi for TCPIP_OWNER_MODULE_BASIC_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCPIP_OWNER_MODULE_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCPIP_OWNER_MODULE_BASIC_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCPIP_OWNER_MODULE_BASIC_INFO {}
impl ::core::default::Default for TCPIP_OWNER_MODULE_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TCPIP_OWNER_MODULE_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCPIP_OWNER_MODULE_INFO_BASIC: TCPIP_OWNER_MODULE_INFO_CLASS = TCPIP_OWNER_MODULE_INFO_CLASS(0i32);
impl ::core::marker::Copy for TCPIP_OWNER_MODULE_INFO_CLASS {}
impl ::core::clone::Clone for TCPIP_OWNER_MODULE_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TCPIP_OWNER_MODULE_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TCPIP_OWNER_MODULE_INFO_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TCPIP_OWNER_MODULE_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCPIP_OWNER_MODULE_INFO_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCPIP_OWNING_MODULE_SIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TCP_BOOLEAN_OPTIONAL(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpBoolOptDisabled: TCP_BOOLEAN_OPTIONAL = TCP_BOOLEAN_OPTIONAL(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpBoolOptEnabled: TCP_BOOLEAN_OPTIONAL = TCP_BOOLEAN_OPTIONAL(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpBoolOptUnchanged: TCP_BOOLEAN_OPTIONAL = TCP_BOOLEAN_OPTIONAL(-1i32);
impl ::core::marker::Copy for TCP_BOOLEAN_OPTIONAL {}
impl ::core::clone::Clone for TCP_BOOLEAN_OPTIONAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TCP_BOOLEAN_OPTIONAL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TCP_BOOLEAN_OPTIONAL {
    type Abi = Self;
}
impl ::core::fmt::Debug for TCP_BOOLEAN_OPTIONAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_BOOLEAN_OPTIONAL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TCP_CONNECTION_OFFLOAD_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionOffloadStateInHost: TCP_CONNECTION_OFFLOAD_STATE = TCP_CONNECTION_OFFLOAD_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionOffloadStateOffloading: TCP_CONNECTION_OFFLOAD_STATE = TCP_CONNECTION_OFFLOAD_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionOffloadStateOffloaded: TCP_CONNECTION_OFFLOAD_STATE = TCP_CONNECTION_OFFLOAD_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionOffloadStateUploading: TCP_CONNECTION_OFFLOAD_STATE = TCP_CONNECTION_OFFLOAD_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionOffloadStateMax: TCP_CONNECTION_OFFLOAD_STATE = TCP_CONNECTION_OFFLOAD_STATE(4i32);
impl ::core::marker::Copy for TCP_CONNECTION_OFFLOAD_STATE {}
impl ::core::clone::Clone for TCP_CONNECTION_OFFLOAD_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TCP_CONNECTION_OFFLOAD_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TCP_CONNECTION_OFFLOAD_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TCP_CONNECTION_OFFLOAD_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_CONNECTION_OFFLOAD_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_BANDWIDTH_ROD_v0 {
    pub OutboundBandwidth: u64,
    pub InboundBandwidth: u64,
    pub OutboundInstability: u64,
    pub InboundInstability: u64,
    pub OutboundBandwidthPeaked: super::super::Foundation::BOOLEAN,
    pub InboundBandwidthPeaked: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_BANDWIDTH_ROD_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_BANDWIDTH_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_BANDWIDTH_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_BANDWIDTH_ROD_v0").field("OutboundBandwidth", &self.OutboundBandwidth).field("InboundBandwidth", &self.InboundBandwidth).field("OutboundInstability", &self.OutboundInstability).field("InboundInstability", &self.InboundInstability).field("OutboundBandwidthPeaked", &self.OutboundBandwidthPeaked).field("InboundBandwidthPeaked", &self.InboundBandwidthPeaked).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCP_ESTATS_BANDWIDTH_ROD_v0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_BANDWIDTH_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_BANDWIDTH_ROD_v0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_BANDWIDTH_ROD_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_BANDWIDTH_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_BANDWIDTH_RW_v0 {
    pub EnableCollectionOutbound: TCP_BOOLEAN_OPTIONAL,
    pub EnableCollectionInbound: TCP_BOOLEAN_OPTIONAL,
}
impl ::core::marker::Copy for TCP_ESTATS_BANDWIDTH_RW_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_BANDWIDTH_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_ESTATS_BANDWIDTH_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_BANDWIDTH_RW_v0").field("EnableCollectionOutbound", &self.EnableCollectionOutbound).field("EnableCollectionInbound", &self.EnableCollectionInbound).finish()
    }
}
unsafe impl ::windows::core::Abi for TCP_ESTATS_BANDWIDTH_RW_v0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCP_ESTATS_BANDWIDTH_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_BANDWIDTH_RW_v0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_BANDWIDTH_RW_v0 {}
impl ::core::default::Default for TCP_ESTATS_BANDWIDTH_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_DATA_ROD_v0 {
    pub DataBytesOut: u64,
    pub DataSegsOut: u64,
    pub DataBytesIn: u64,
    pub DataSegsIn: u64,
    pub SegsOut: u64,
    pub SegsIn: u64,
    pub SoftErrors: u32,
    pub SoftErrorReason: u32,
    pub SndUna: u32,
    pub SndNxt: u32,
    pub SndMax: u32,
    pub ThruBytesAcked: u64,
    pub RcvNxt: u32,
    pub ThruBytesReceived: u64,
}
impl ::core::marker::Copy for TCP_ESTATS_DATA_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_DATA_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_ESTATS_DATA_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_DATA_ROD_v0")
            .field("DataBytesOut", &self.DataBytesOut)
            .field("DataSegsOut", &self.DataSegsOut)
            .field("DataBytesIn", &self.DataBytesIn)
            .field("DataSegsIn", &self.DataSegsIn)
            .field("SegsOut", &self.SegsOut)
            .field("SegsIn", &self.SegsIn)
            .field("SoftErrors", &self.SoftErrors)
            .field("SoftErrorReason", &self.SoftErrorReason)
            .field("SndUna", &self.SndUna)
            .field("SndNxt", &self.SndNxt)
            .field("SndMax", &self.SndMax)
            .field("ThruBytesAcked", &self.ThruBytesAcked)
            .field("RcvNxt", &self.RcvNxt)
            .field("ThruBytesReceived", &self.ThruBytesReceived)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for TCP_ESTATS_DATA_ROD_v0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCP_ESTATS_DATA_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_DATA_ROD_v0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_DATA_ROD_v0 {}
impl ::core::default::Default for TCP_ESTATS_DATA_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_DATA_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_DATA_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_DATA_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_DATA_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_DATA_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCP_ESTATS_DATA_RW_v0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_DATA_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_DATA_RW_v0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_DATA_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_DATA_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_FINE_RTT_ROD_v0 {
    pub RttVar: u32,
    pub MaxRtt: u32,
    pub MinRtt: u32,
    pub SumRtt: u32,
}
impl ::core::marker::Copy for TCP_ESTATS_FINE_RTT_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_FINE_RTT_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_ESTATS_FINE_RTT_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_FINE_RTT_ROD_v0").field("RttVar", &self.RttVar).field("MaxRtt", &self.MaxRtt).field("MinRtt", &self.MinRtt).field("SumRtt", &self.SumRtt).finish()
    }
}
unsafe impl ::windows::core::Abi for TCP_ESTATS_FINE_RTT_ROD_v0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCP_ESTATS_FINE_RTT_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_FINE_RTT_ROD_v0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_FINE_RTT_ROD_v0 {}
impl ::core::default::Default for TCP_ESTATS_FINE_RTT_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_FINE_RTT_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_FINE_RTT_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_FINE_RTT_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_FINE_RTT_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_FINE_RTT_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCP_ESTATS_FINE_RTT_RW_v0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_FINE_RTT_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_FINE_RTT_RW_v0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_FINE_RTT_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_FINE_RTT_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_OBS_REC_ROD_v0 {
    pub CurRwinRcvd: u32,
    pub MaxRwinRcvd: u32,
    pub MinRwinRcvd: u32,
    pub WinScaleRcvd: u8,
}
impl ::core::marker::Copy for TCP_ESTATS_OBS_REC_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_OBS_REC_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_ESTATS_OBS_REC_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_OBS_REC_ROD_v0").field("CurRwinRcvd", &self.CurRwinRcvd).field("MaxRwinRcvd", &self.MaxRwinRcvd).field("MinRwinRcvd", &self.MinRwinRcvd).field("WinScaleRcvd", &self.WinScaleRcvd).finish()
    }
}
unsafe impl ::windows::core::Abi for TCP_ESTATS_OBS_REC_ROD_v0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCP_ESTATS_OBS_REC_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_OBS_REC_ROD_v0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_OBS_REC_ROD_v0 {}
impl ::core::default::Default for TCP_ESTATS_OBS_REC_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_OBS_REC_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_OBS_REC_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_OBS_REC_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_OBS_REC_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_OBS_REC_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCP_ESTATS_OBS_REC_RW_v0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_OBS_REC_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_OBS_REC_RW_v0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_OBS_REC_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_OBS_REC_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_PATH_ROD_v0 {
    pub FastRetran: u32,
    pub Timeouts: u32,
    pub SubsequentTimeouts: u32,
    pub CurTimeoutCount: u32,
    pub AbruptTimeouts: u32,
    pub PktsRetrans: u32,
    pub BytesRetrans: u32,
    pub DupAcksIn: u32,
    pub SacksRcvd: u32,
    pub SackBlocksRcvd: u32,
    pub CongSignals: u32,
    pub PreCongSumCwnd: u32,
    pub PreCongSumRtt: u32,
    pub PostCongSumRtt: u32,
    pub PostCongCountRtt: u32,
    pub EcnSignals: u32,
    pub EceRcvd: u32,
    pub SendStall: u32,
    pub QuenchRcvd: u32,
    pub RetranThresh: u32,
    pub SndDupAckEpisodes: u32,
    pub SumBytesReordered: u32,
    pub NonRecovDa: u32,
    pub NonRecovDaEpisodes: u32,
    pub AckAfterFr: u32,
    pub DsackDups: u32,
    pub SampleRtt: u32,
    pub SmoothedRtt: u32,
    pub RttVar: u32,
    pub MaxRtt: u32,
    pub MinRtt: u32,
    pub SumRtt: u32,
    pub CountRtt: u32,
    pub CurRto: u32,
    pub MaxRto: u32,
    pub MinRto: u32,
    pub CurMss: u32,
    pub MaxMss: u32,
    pub MinMss: u32,
    pub SpuriousRtoDetections: u32,
}
impl ::core::marker::Copy for TCP_ESTATS_PATH_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_PATH_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_ESTATS_PATH_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_PATH_ROD_v0")
            .field("FastRetran", &self.FastRetran)
            .field("Timeouts", &self.Timeouts)
            .field("SubsequentTimeouts", &self.SubsequentTimeouts)
            .field("CurTimeoutCount", &self.CurTimeoutCount)
            .field("AbruptTimeouts", &self.AbruptTimeouts)
            .field("PktsRetrans", &self.PktsRetrans)
            .field("BytesRetrans", &self.BytesRetrans)
            .field("DupAcksIn", &self.DupAcksIn)
            .field("SacksRcvd", &self.SacksRcvd)
            .field("SackBlocksRcvd", &self.SackBlocksRcvd)
            .field("CongSignals", &self.CongSignals)
            .field("PreCongSumCwnd", &self.PreCongSumCwnd)
            .field("PreCongSumRtt", &self.PreCongSumRtt)
            .field("PostCongSumRtt", &self.PostCongSumRtt)
            .field("PostCongCountRtt", &self.PostCongCountRtt)
            .field("EcnSignals", &self.EcnSignals)
            .field("EceRcvd", &self.EceRcvd)
            .field("SendStall", &self.SendStall)
            .field("QuenchRcvd", &self.QuenchRcvd)
            .field("RetranThresh", &self.RetranThresh)
            .field("SndDupAckEpisodes", &self.SndDupAckEpisodes)
            .field("SumBytesReordered", &self.SumBytesReordered)
            .field("NonRecovDa", &self.NonRecovDa)
            .field("NonRecovDaEpisodes", &self.NonRecovDaEpisodes)
            .field("AckAfterFr", &self.AckAfterFr)
            .field("DsackDups", &self.DsackDups)
            .field("SampleRtt", &self.SampleRtt)
            .field("SmoothedRtt", &self.SmoothedRtt)
            .field("RttVar", &self.RttVar)
            .field("MaxRtt", &self.MaxRtt)
            .field("MinRtt", &self.MinRtt)
            .field("SumRtt", &self.SumRtt)
            .field("CountRtt", &self.CountRtt)
            .field("CurRto", &self.CurRto)
            .field("MaxRto", &self.MaxRto)
            .field("MinRto", &self.MinRto)
            .field("CurMss", &self.CurMss)
            .field("MaxMss", &self.MaxMss)
            .field("MinMss", &self.MinMss)
            .field("SpuriousRtoDetections", &self.SpuriousRtoDetections)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for TCP_ESTATS_PATH_ROD_v0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCP_ESTATS_PATH_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_PATH_ROD_v0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_PATH_ROD_v0 {}
impl ::core::default::Default for TCP_ESTATS_PATH_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_PATH_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_PATH_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_PATH_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_PATH_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_PATH_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCP_ESTATS_PATH_RW_v0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_PATH_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_PATH_RW_v0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_PATH_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_PATH_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_REC_ROD_v0 {
    pub CurRwinSent: u32,
    pub MaxRwinSent: u32,
    pub MinRwinSent: u32,
    pub LimRwin: u32,
    pub DupAckEpisodes: u32,
    pub DupAcksOut: u32,
    pub CeRcvd: u32,
    pub EcnSent: u32,
    pub EcnNoncesRcvd: u32,
    pub CurReasmQueue: u32,
    pub MaxReasmQueue: u32,
    pub CurAppRQueue: usize,
    pub MaxAppRQueue: usize,
    pub WinScaleSent: u8,
}
impl ::core::marker::Copy for TCP_ESTATS_REC_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_REC_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_ESTATS_REC_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_REC_ROD_v0")
            .field("CurRwinSent", &self.CurRwinSent)
            .field("MaxRwinSent", &self.MaxRwinSent)
            .field("MinRwinSent", &self.MinRwinSent)
            .field("LimRwin", &self.LimRwin)
            .field("DupAckEpisodes", &self.DupAckEpisodes)
            .field("DupAcksOut", &self.DupAcksOut)
            .field("CeRcvd", &self.CeRcvd)
            .field("EcnSent", &self.EcnSent)
            .field("EcnNoncesRcvd", &self.EcnNoncesRcvd)
            .field("CurReasmQueue", &self.CurReasmQueue)
            .field("MaxReasmQueue", &self.MaxReasmQueue)
            .field("CurAppRQueue", &self.CurAppRQueue)
            .field("MaxAppRQueue", &self.MaxAppRQueue)
            .field("WinScaleSent", &self.WinScaleSent)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for TCP_ESTATS_REC_ROD_v0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCP_ESTATS_REC_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_REC_ROD_v0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_REC_ROD_v0 {}
impl ::core::default::Default for TCP_ESTATS_REC_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_REC_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_REC_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_REC_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_REC_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_REC_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCP_ESTATS_REC_RW_v0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_REC_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_REC_RW_v0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_REC_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_REC_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_SEND_BUFF_ROD_v0 {
    pub CurRetxQueue: usize,
    pub MaxRetxQueue: usize,
    pub CurAppWQueue: usize,
    pub MaxAppWQueue: usize,
}
impl ::core::marker::Copy for TCP_ESTATS_SEND_BUFF_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_SEND_BUFF_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_ESTATS_SEND_BUFF_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_SEND_BUFF_ROD_v0").field("CurRetxQueue", &self.CurRetxQueue).field("MaxRetxQueue", &self.MaxRetxQueue).field("CurAppWQueue", &self.CurAppWQueue).field("MaxAppWQueue", &self.MaxAppWQueue).finish()
    }
}
unsafe impl ::windows::core::Abi for TCP_ESTATS_SEND_BUFF_ROD_v0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCP_ESTATS_SEND_BUFF_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_SEND_BUFF_ROD_v0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_SEND_BUFF_ROD_v0 {}
impl ::core::default::Default for TCP_ESTATS_SEND_BUFF_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_SEND_BUFF_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_SEND_BUFF_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_SEND_BUFF_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_SEND_BUFF_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_SEND_BUFF_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCP_ESTATS_SEND_BUFF_RW_v0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_SEND_BUFF_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_SEND_BUFF_RW_v0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_SEND_BUFF_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_SEND_BUFF_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_SND_CONG_ROD_v0 {
    pub SndLimTransRwin: u32,
    pub SndLimTimeRwin: u32,
    pub SndLimBytesRwin: usize,
    pub SndLimTransCwnd: u32,
    pub SndLimTimeCwnd: u32,
    pub SndLimBytesCwnd: usize,
    pub SndLimTransSnd: u32,
    pub SndLimTimeSnd: u32,
    pub SndLimBytesSnd: usize,
    pub SlowStart: u32,
    pub CongAvoid: u32,
    pub OtherReductions: u32,
    pub CurCwnd: u32,
    pub MaxSsCwnd: u32,
    pub MaxCaCwnd: u32,
    pub CurSsthresh: u32,
    pub MaxSsthresh: u32,
    pub MinSsthresh: u32,
}
impl ::core::marker::Copy for TCP_ESTATS_SND_CONG_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_SND_CONG_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_ESTATS_SND_CONG_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_SND_CONG_ROD_v0")
            .field("SndLimTransRwin", &self.SndLimTransRwin)
            .field("SndLimTimeRwin", &self.SndLimTimeRwin)
            .field("SndLimBytesRwin", &self.SndLimBytesRwin)
            .field("SndLimTransCwnd", &self.SndLimTransCwnd)
            .field("SndLimTimeCwnd", &self.SndLimTimeCwnd)
            .field("SndLimBytesCwnd", &self.SndLimBytesCwnd)
            .field("SndLimTransSnd", &self.SndLimTransSnd)
            .field("SndLimTimeSnd", &self.SndLimTimeSnd)
            .field("SndLimBytesSnd", &self.SndLimBytesSnd)
            .field("SlowStart", &self.SlowStart)
            .field("CongAvoid", &self.CongAvoid)
            .field("OtherReductions", &self.OtherReductions)
            .field("CurCwnd", &self.CurCwnd)
            .field("MaxSsCwnd", &self.MaxSsCwnd)
            .field("MaxCaCwnd", &self.MaxCaCwnd)
            .field("CurSsthresh", &self.CurSsthresh)
            .field("MaxSsthresh", &self.MaxSsthresh)
            .field("MinSsthresh", &self.MinSsthresh)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for TCP_ESTATS_SND_CONG_ROD_v0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCP_ESTATS_SND_CONG_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_SND_CONG_ROD_v0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_SND_CONG_ROD_v0 {}
impl ::core::default::Default for TCP_ESTATS_SND_CONG_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_SND_CONG_ROS_v0 {
    pub LimCwnd: u32,
}
impl ::core::marker::Copy for TCP_ESTATS_SND_CONG_ROS_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_SND_CONG_ROS_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_ESTATS_SND_CONG_ROS_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_SND_CONG_ROS_v0").field("LimCwnd", &self.LimCwnd).finish()
    }
}
unsafe impl ::windows::core::Abi for TCP_ESTATS_SND_CONG_ROS_v0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCP_ESTATS_SND_CONG_ROS_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_SND_CONG_ROS_v0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_SND_CONG_ROS_v0 {}
impl ::core::default::Default for TCP_ESTATS_SND_CONG_ROS_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_SND_CONG_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_SND_CONG_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_SND_CONG_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_SND_CONG_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_SND_CONG_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCP_ESTATS_SND_CONG_RW_v0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_SND_CONG_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_SND_CONG_RW_v0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_SND_CONG_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_SND_CONG_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_SYN_OPTS_ROS_v0 {
    pub ActiveOpen: super::super::Foundation::BOOLEAN,
    pub MssRcvd: u32,
    pub MssSent: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_SYN_OPTS_ROS_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_SYN_OPTS_ROS_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_SYN_OPTS_ROS_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_SYN_OPTS_ROS_v0").field("ActiveOpen", &self.ActiveOpen).field("MssRcvd", &self.MssRcvd).field("MssSent", &self.MssSent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCP_ESTATS_SYN_OPTS_ROS_v0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_SYN_OPTS_ROS_v0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCP_ESTATS_SYN_OPTS_ROS_v0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_SYN_OPTS_ROS_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_SYN_OPTS_ROS_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TCP_ESTATS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsSynOpts: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsData: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsSndCong: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsPath: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsSendBuff: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsRec: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsObsRec: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsBandwidth: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsFineRtt: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsMaximum: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(9i32);
impl ::core::marker::Copy for TCP_ESTATS_TYPE {}
impl ::core::clone::Clone for TCP_ESTATS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TCP_ESTATS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TCP_ESTATS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TCP_ESTATS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_ESTATS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_ROW: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TCP_RTO_ALGORITHM(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpRtoAlgorithmOther: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpRtoAlgorithmConstant: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpRtoAlgorithmRsre: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpRtoAlgorithmVanj: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_RTO_OTHER: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_RTO_CONSTANT: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_RTO_RSRE: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_RTO_VANJ: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(4i32);
impl ::core::marker::Copy for TCP_RTO_ALGORITHM {}
impl ::core::clone::Clone for TCP_RTO_ALGORITHM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TCP_RTO_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TCP_RTO_ALGORITHM {
    type Abi = Self;
}
impl ::core::fmt::Debug for TCP_RTO_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_RTO_ALGORITHM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TCP_SOFT_ERROR(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorNone: TCP_SOFT_ERROR = TCP_SOFT_ERROR(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorBelowDataWindow: TCP_SOFT_ERROR = TCP_SOFT_ERROR(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorAboveDataWindow: TCP_SOFT_ERROR = TCP_SOFT_ERROR(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorBelowAckWindow: TCP_SOFT_ERROR = TCP_SOFT_ERROR(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorAboveAckWindow: TCP_SOFT_ERROR = TCP_SOFT_ERROR(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorBelowTsWindow: TCP_SOFT_ERROR = TCP_SOFT_ERROR(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorAboveTsWindow: TCP_SOFT_ERROR = TCP_SOFT_ERROR(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorDataChecksumError: TCP_SOFT_ERROR = TCP_SOFT_ERROR(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorDataLengthError: TCP_SOFT_ERROR = TCP_SOFT_ERROR(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorMaxSoftError: TCP_SOFT_ERROR = TCP_SOFT_ERROR(9i32);
impl ::core::marker::Copy for TCP_SOFT_ERROR {}
impl ::core::clone::Clone for TCP_SOFT_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TCP_SOFT_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TCP_SOFT_ERROR {
    type Abi = Self;
}
impl ::core::fmt::Debug for TCP_SOFT_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_SOFT_ERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_STATS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TCP_TABLE_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_BASIC_LISTENER: TCP_TABLE_CLASS = TCP_TABLE_CLASS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_BASIC_CONNECTIONS: TCP_TABLE_CLASS = TCP_TABLE_CLASS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_BASIC_ALL: TCP_TABLE_CLASS = TCP_TABLE_CLASS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_OWNER_PID_LISTENER: TCP_TABLE_CLASS = TCP_TABLE_CLASS(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_OWNER_PID_CONNECTIONS: TCP_TABLE_CLASS = TCP_TABLE_CLASS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_OWNER_PID_ALL: TCP_TABLE_CLASS = TCP_TABLE_CLASS(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_OWNER_MODULE_LISTENER: TCP_TABLE_CLASS = TCP_TABLE_CLASS(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_OWNER_MODULE_CONNECTIONS: TCP_TABLE_CLASS = TCP_TABLE_CLASS(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_OWNER_MODULE_ALL: TCP_TABLE_CLASS = TCP_TABLE_CLASS(8i32);
impl ::core::marker::Copy for TCP_TABLE_CLASS {}
impl ::core::clone::Clone for TCP_TABLE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TCP_TABLE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TCP_TABLE_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TCP_TABLE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_TABLE_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TUNNEL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_NONE: TUNNEL_TYPE = TUNNEL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_OTHER: TUNNEL_TYPE = TUNNEL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_DIRECT: TUNNEL_TYPE = TUNNEL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_6TO4: TUNNEL_TYPE = TUNNEL_TYPE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_ISATAP: TUNNEL_TYPE = TUNNEL_TYPE(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_TEREDO: TUNNEL_TYPE = TUNNEL_TYPE(14i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_IPHTTPS: TUNNEL_TYPE = TUNNEL_TYPE(15i32);
impl ::core::marker::Copy for TUNNEL_TYPE {}
impl ::core::clone::Clone for TUNNEL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TUNNEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TUNNEL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TUNNEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TUNNEL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP6_STATS: u32 = 37u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP_ROW: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP_STATS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP_TABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UDP_TABLE_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP_TABLE_BASIC: UDP_TABLE_CLASS = UDP_TABLE_CLASS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP_TABLE_OWNER_PID: UDP_TABLE_CLASS = UDP_TABLE_CLASS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP_TABLE_OWNER_MODULE: UDP_TABLE_CLASS = UDP_TABLE_CLASS(2i32);
impl ::core::marker::Copy for UDP_TABLE_CLASS {}
impl ::core::clone::Clone for UDP_TABLE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UDP_TABLE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UDP_TABLE_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UDP_TABLE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDP_TABLE_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn UnenableRouter(poverlapped: *const super::super::System::IO::OVERLAPPED, lpdwenablecount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnenableRouter(poverlapped: *const super::super::System::IO::OVERLAPPED, lpdwenablecount: *mut u32) -> u32;
        }
        ::core::mem::transmute(UnenableRouter(::core::mem::transmute(poverlapped), ::core::mem::transmute(lpdwenablecount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn UnregisterInterfaceTimestampConfigChange<'a, Param0: ::windows::core::IntoParam<'a, HIFTIMESTAMPCHANGE>>(notificationhandle: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterInterfaceTimestampConfigChange(notificationhandle: HIFTIMESTAMPCHANGE);
        }
        UnregisterInterfaceTimestampConfigChange(notificationhandle.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct arp_send_reply {
    pub DestAddress: u32,
    pub SrcAddress: u32,
}
impl ::core::marker::Copy for arp_send_reply {}
impl ::core::clone::Clone for arp_send_reply {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for arp_send_reply {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("arp_send_reply").field("DestAddress", &self.DestAddress).field("SrcAddress", &self.SrcAddress).finish()
    }
}
unsafe impl ::windows::core::Abi for arp_send_reply {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for arp_send_reply {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<arp_send_reply>()) == 0 }
    }
}
impl ::core::cmp::Eq for arp_send_reply {}
impl ::core::default::Default for arp_send_reply {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct icmp_echo_reply {
    pub Address: u32,
    pub Status: u32,
    pub RoundTripTime: u32,
    pub DataSize: u16,
    pub Reserved: u16,
    pub Data: *mut ::core::ffi::c_void,
    pub Options: ip_option_information,
}
impl ::core::marker::Copy for icmp_echo_reply {}
impl ::core::clone::Clone for icmp_echo_reply {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for icmp_echo_reply {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("icmp_echo_reply").field("Address", &self.Address).field("Status", &self.Status).field("RoundTripTime", &self.RoundTripTime).field("DataSize", &self.DataSize).field("Reserved", &self.Reserved).field("Data", &self.Data).field("Options", &self.Options).finish()
    }
}
unsafe impl ::windows::core::Abi for icmp_echo_reply {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for icmp_echo_reply {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<icmp_echo_reply>()) == 0 }
    }
}
impl ::core::cmp::Eq for icmp_echo_reply {}
impl ::core::default::Default for icmp_echo_reply {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct icmp_echo_reply32 {
    pub Address: u32,
    pub Status: u32,
    pub RoundTripTime: u32,
    pub DataSize: u16,
    pub Reserved: u16,
    pub Data: *mut ::core::ffi::c_void,
    pub Options: ip_option_information32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for icmp_echo_reply32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for icmp_echo_reply32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for icmp_echo_reply32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("icmp_echo_reply32").field("Address", &self.Address).field("Status", &self.Status).field("RoundTripTime", &self.RoundTripTime).field("DataSize", &self.DataSize).field("Reserved", &self.Reserved).field("Data", &self.Data).field("Options", &self.Options).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for icmp_echo_reply32 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for icmp_echo_reply32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<icmp_echo_reply32>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for icmp_echo_reply32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for icmp_echo_reply32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct icmpv6_echo_reply_lh {
    pub Address: IPV6_ADDRESS_EX,
    pub Status: u32,
    pub RoundTripTime: u32,
}
impl ::core::marker::Copy for icmpv6_echo_reply_lh {}
impl ::core::clone::Clone for icmpv6_echo_reply_lh {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for icmpv6_echo_reply_lh {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for icmpv6_echo_reply_lh {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<icmpv6_echo_reply_lh>()) == 0 }
    }
}
impl ::core::cmp::Eq for icmpv6_echo_reply_lh {}
impl ::core::default::Default for icmpv6_echo_reply_lh {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn if_indextoname(interfaceindex: u32, interfacename: &mut [u8; 256]) -> ::windows::core::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn if_indextoname(interfaceindex: u32, interfacename: ::windows::core::PSTR) -> ::windows::core::PSTR;
        }
        ::core::mem::transmute(if_indextoname(::core::mem::transmute(interfaceindex), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(interfacename))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[inline]
pub unsafe fn if_nametoindex<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(interfacename: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn if_nametoindex(interfacename: ::windows::core::PCSTR) -> u32;
        }
        ::core::mem::transmute(if_nametoindex(interfacename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct ip_interface_name_info_w2ksp1 {
    pub Index: u32,
    pub MediaType: u32,
    pub ConnectionType: u8,
    pub AccessType: u8,
    pub DeviceGuid: ::windows::core::GUID,
    pub InterfaceGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for ip_interface_name_info_w2ksp1 {}
impl ::core::clone::Clone for ip_interface_name_info_w2ksp1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ip_interface_name_info_w2ksp1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ip_interface_name_info_w2ksp1").field("Index", &self.Index).field("MediaType", &self.MediaType).field("ConnectionType", &self.ConnectionType).field("AccessType", &self.AccessType).field("DeviceGuid", &self.DeviceGuid).field("InterfaceGuid", &self.InterfaceGuid).finish()
    }
}
unsafe impl ::windows::core::Abi for ip_interface_name_info_w2ksp1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ip_interface_name_info_w2ksp1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ip_interface_name_info_w2ksp1>()) == 0 }
    }
}
impl ::core::cmp::Eq for ip_interface_name_info_w2ksp1 {}
impl ::core::default::Default for ip_interface_name_info_w2ksp1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct ip_option_information {
    pub Ttl: u8,
    pub Tos: u8,
    pub Flags: u8,
    pub OptionsSize: u8,
    pub OptionsData: *mut u8,
}
impl ::core::marker::Copy for ip_option_information {}
impl ::core::clone::Clone for ip_option_information {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ip_option_information {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ip_option_information").field("Ttl", &self.Ttl).field("Tos", &self.Tos).field("Flags", &self.Flags).field("OptionsSize", &self.OptionsSize).field("OptionsData", &self.OptionsData).finish()
    }
}
unsafe impl ::windows::core::Abi for ip_option_information {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ip_option_information {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ip_option_information>()) == 0 }
    }
}
impl ::core::cmp::Eq for ip_option_information {}
impl ::core::default::Default for ip_option_information {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct ip_option_information32 {
    pub Ttl: u8,
    pub Tos: u8,
    pub Flags: u8,
    pub OptionsSize: u8,
    pub OptionsData: *mut u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for ip_option_information32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for ip_option_information32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for ip_option_information32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ip_option_information32").field("Ttl", &self.Ttl).field("Tos", &self.Tos).field("Flags", &self.Flags).field("OptionsSize", &self.OptionsSize).field("OptionsData", &self.OptionsData).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for ip_option_information32 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for ip_option_information32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ip_option_information32>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for ip_option_information32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for ip_option_information32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct tcp_reserve_port_range {
    pub UpperRange: u16,
    pub LowerRange: u16,
}
impl ::core::marker::Copy for tcp_reserve_port_range {}
impl ::core::clone::Clone for tcp_reserve_port_range {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tcp_reserve_port_range {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_reserve_port_range").field("UpperRange", &self.UpperRange).field("LowerRange", &self.LowerRange).finish()
    }
}
unsafe impl ::windows::core::Abi for tcp_reserve_port_range {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_reserve_port_range {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_reserve_port_range>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_reserve_port_range {}
impl ::core::default::Default for tcp_reserve_port_range {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
