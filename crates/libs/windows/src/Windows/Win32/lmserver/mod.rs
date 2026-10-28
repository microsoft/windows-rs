#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetServerComputerNameAdd<P0, P1, P2>(servername: P0, emulateddomainname: P1, emulatedservername: P2) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerComputerNameAdd(servername : windows_core::PCWSTR, emulateddomainname : windows_core::PCWSTR, emulatedservername : windows_core::PCWSTR) -> super::DWORD);
    unsafe { NetServerComputerNameAdd(servername.param().abi(), emulateddomainname.param().abi(), emulatedservername.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetServerComputerNameDel<P0, P1>(servername: P0, emulatedservername: P1) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerComputerNameDel(servername : windows_core::PCWSTR, emulatedservername : windows_core::PCWSTR) -> super::DWORD);
    unsafe { NetServerComputerNameDel(servername.param().abi(), emulatedservername.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetServerDiskEnum<P0>(servername: P0, level: super::DWORD, bufptr: *mut super::LPBYTE, prefmaxlen: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, resume_handle: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerDiskEnum(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resume_handle : super::LPDWORD) -> super::DWORD);
    unsafe { NetServerDiskEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetServerEnum<P0, P7>(servername: P0, level: super::DWORD, bufptr: *mut super::LPBYTE, prefmaxlen: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, servertype: super::DWORD, domain: P7, resume_handle: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerEnum(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, servertype : super::DWORD, domain : windows_core::PCWSTR, resume_handle : super::LPDWORD) -> super::DWORD);
    unsafe { NetServerEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, servertype, domain.param().abi(), resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetServerGetInfo<P0>(servername: P0, level: super::DWORD, bufptr: *mut super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerGetInfo(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetServerGetInfo(servername.param().abi(), level, bufptr as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetServerSetInfo<P0>(servername: P0, level: super::DWORD, buf: super::LPBYTE, parmerror: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerSetInfo(servername : windows_core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parmerror : super::LPDWORD) -> super::DWORD);
    unsafe { NetServerSetInfo(servername.param().abi(), level, buf as _, parmerror.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetServerTransportAdd<P0>(servername: P0, level: super::DWORD, bufptr: super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerTransportAdd(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : super::LPBYTE) -> super::DWORD);
    unsafe { NetServerTransportAdd(servername.param().abi(), level, bufptr) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetServerTransportAddEx<P0>(servername: P0, level: super::DWORD, bufptr: super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerTransportAddEx(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : super::LPBYTE) -> super::DWORD);
    unsafe { NetServerTransportAddEx(servername.param().abi(), level, bufptr as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetServerTransportDel<P0>(servername: P0, level: super::DWORD, bufptr: super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerTransportDel(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : super::LPBYTE) -> super::DWORD);
    unsafe { NetServerTransportDel(servername.param().abi(), level, bufptr as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetServerTransportEnum<P0>(servername: P0, level: super::DWORD, bufptr: *mut super::LPBYTE, prefmaxlen: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, resume_handle: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerTransportEnum(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resume_handle : super::LPDWORD) -> super::DWORD);
    unsafe { NetServerTransportEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winsvc"))]
#[inline]
pub unsafe fn SetServiceBits(hservicestatus: super::SERVICE_STATUS_HANDLE, dwservicebits: super::DWORD, bsetbitson: bool, bupdateimmediately: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetServiceBits(hservicestatus : super::SERVICE_STATUS_HANDLE, dwservicebits : super::DWORD, bsetbitson : windows_core::BOOL, bupdateimmediately : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetServiceBits(hservicestatus as _, dwservicebits, bsetbitson.into(), bupdateimmediately.into()) }
}
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_100 = *mut SERVER_INFO_100;
pub type LPSERVER_INFO_1005 = *mut SERVER_INFO_1005;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_101 = *mut SERVER_INFO_101;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1010 = *mut SERVER_INFO_1010;
pub type LPSERVER_INFO_1016 = *mut SERVER_INFO_1016;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1017 = *mut SERVER_INFO_1017;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1018 = *mut SERVER_INFO_1018;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_102 = *mut SERVER_INFO_102;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_103 = *mut SERVER_INFO_103;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1107 = *mut SERVER_INFO_1107;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1501 = *mut SERVER_INFO_1501;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1502 = *mut SERVER_INFO_1502;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1503 = *mut SERVER_INFO_1503;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1506 = *mut SERVER_INFO_1506;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1509 = *mut SERVER_INFO_1509;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1510 = *mut SERVER_INFO_1510;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1511 = *mut SERVER_INFO_1511;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1512 = *mut SERVER_INFO_1512;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1513 = *mut SERVER_INFO_1513;
pub type LPSERVER_INFO_1514 = *mut SERVER_INFO_1514;
pub type LPSERVER_INFO_1515 = *mut SERVER_INFO_1515;
pub type LPSERVER_INFO_1516 = *mut SERVER_INFO_1516;
pub type LPSERVER_INFO_1518 = *mut SERVER_INFO_1518;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1520 = *mut SERVER_INFO_1520;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1521 = *mut SERVER_INFO_1521;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1522 = *mut SERVER_INFO_1522;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1523 = *mut SERVER_INFO_1523;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1524 = *mut SERVER_INFO_1524;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1525 = *mut SERVER_INFO_1525;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1528 = *mut SERVER_INFO_1528;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1529 = *mut SERVER_INFO_1529;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1530 = *mut SERVER_INFO_1530;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1533 = *mut SERVER_INFO_1533;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1534 = *mut SERVER_INFO_1534;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1535 = *mut SERVER_INFO_1535;
pub type LPSERVER_INFO_1536 = *mut SERVER_INFO_1536;
pub type LPSERVER_INFO_1537 = *mut SERVER_INFO_1537;
pub type LPSERVER_INFO_1538 = *mut SERVER_INFO_1538;
pub type LPSERVER_INFO_1539 = *mut SERVER_INFO_1539;
pub type LPSERVER_INFO_1540 = *mut SERVER_INFO_1540;
pub type LPSERVER_INFO_1541 = *mut SERVER_INFO_1541;
pub type LPSERVER_INFO_1542 = *mut SERVER_INFO_1542;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1543 = *mut SERVER_INFO_1543;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1544 = *mut SERVER_INFO_1544;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1545 = *mut SERVER_INFO_1545;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1546 = *mut SERVER_INFO_1546;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1547 = *mut SERVER_INFO_1547;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1548 = *mut SERVER_INFO_1548;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1549 = *mut SERVER_INFO_1549;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1550 = *mut SERVER_INFO_1550;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1552 = *mut SERVER_INFO_1552;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1553 = *mut SERVER_INFO_1553;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1554 = *mut SERVER_INFO_1554;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1555 = *mut SERVER_INFO_1555;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1556 = *mut SERVER_INFO_1556;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1557 = *mut SERVER_INFO_1557;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1560 = *mut SERVER_INFO_1560;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1561 = *mut SERVER_INFO_1561;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1562 = *mut SERVER_INFO_1562;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1563 = *mut SERVER_INFO_1563;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1564 = *mut SERVER_INFO_1564;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1565 = *mut SERVER_INFO_1565;
pub type LPSERVER_INFO_1566 = *mut SERVER_INFO_1566;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1567 = *mut SERVER_INFO_1567;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1568 = *mut SERVER_INFO_1568;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1569 = *mut SERVER_INFO_1569;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1570 = *mut SERVER_INFO_1570;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1571 = *mut SERVER_INFO_1571;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1572 = *mut SERVER_INFO_1572;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1573 = *mut SERVER_INFO_1573;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1574 = *mut SERVER_INFO_1574;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1575 = *mut SERVER_INFO_1575;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1576 = *mut SERVER_INFO_1576;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1577 = *mut SERVER_INFO_1577;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1578 = *mut SERVER_INFO_1578;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1579 = *mut SERVER_INFO_1579;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1580 = *mut SERVER_INFO_1580;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1581 = *mut SERVER_INFO_1581;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1582 = *mut SERVER_INFO_1582;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1583 = *mut SERVER_INFO_1583;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1584 = *mut SERVER_INFO_1584;
pub type LPSERVER_INFO_1585 = *mut SERVER_INFO_1585;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1586 = *mut SERVER_INFO_1586;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1587 = *mut SERVER_INFO_1587;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1588 = *mut SERVER_INFO_1588;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1590 = *mut SERVER_INFO_1590;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1591 = *mut SERVER_INFO_1591;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1592 = *mut SERVER_INFO_1592;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1593 = *mut SERVER_INFO_1593;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1594 = *mut SERVER_INFO_1594;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1595 = *mut SERVER_INFO_1595;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1596 = *mut SERVER_INFO_1596;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1597 = *mut SERVER_INFO_1597;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1598 = *mut SERVER_INFO_1598;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1599 = *mut SERVER_INFO_1599;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1600 = *mut SERVER_INFO_1600;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_1601 = *mut SERVER_INFO_1601;
pub type LPSERVER_INFO_1602 = *mut SERVER_INFO_1602;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_402 = *mut SERVER_INFO_402;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_403 = *mut SERVER_INFO_403;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_502 = *mut SERVER_INFO_502;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_503 = *mut SERVER_INFO_503;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_598 = *mut SERVER_INFO_598;
#[cfg(feature = "shared")]
pub type LPSERVER_INFO_599 = *mut SERVER_INFO_599;
#[cfg(feature = "shared")]
pub type LPSERVER_TRANSPORT_INFO_0 = *mut SERVER_TRANSPORT_INFO_0;
#[cfg(feature = "shared")]
pub type LPSERVER_TRANSPORT_INFO_1 = *mut SERVER_TRANSPORT_INFO_1;
#[cfg(feature = "shared")]
pub type LPSERVER_TRANSPORT_INFO_2 = *mut SERVER_TRANSPORT_INFO_2;
#[cfg(feature = "shared")]
pub type LPSERVER_TRANSPORT_INFO_3 = *mut SERVER_TRANSPORT_INFO_3;
pub const MAJOR_VERSION_MASK: i32 = 15;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_100 = *mut SERVER_INFO_100;
pub type PSERVER_INFO_1005 = *mut SERVER_INFO_1005;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_101 = *mut SERVER_INFO_101;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1010 = *mut SERVER_INFO_1010;
pub type PSERVER_INFO_1016 = *mut SERVER_INFO_1016;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1017 = *mut SERVER_INFO_1017;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1018 = *mut SERVER_INFO_1018;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_102 = *mut SERVER_INFO_102;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_103 = *mut SERVER_INFO_103;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1107 = *mut SERVER_INFO_1107;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1501 = *mut SERVER_INFO_1501;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1502 = *mut SERVER_INFO_1502;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1503 = *mut SERVER_INFO_1503;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1506 = *mut SERVER_INFO_1506;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1509 = *mut SERVER_INFO_1509;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1510 = *mut SERVER_INFO_1510;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1511 = *mut SERVER_INFO_1511;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1512 = *mut SERVER_INFO_1512;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1513 = *mut SERVER_INFO_1513;
pub type PSERVER_INFO_1514 = *mut SERVER_INFO_1514;
pub type PSERVER_INFO_1515 = *mut SERVER_INFO_1515;
pub type PSERVER_INFO_1516 = *mut SERVER_INFO_1516;
pub type PSERVER_INFO_1518 = *mut SERVER_INFO_1518;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1520 = *mut SERVER_INFO_1520;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1521 = *mut SERVER_INFO_1521;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1522 = *mut SERVER_INFO_1522;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1523 = *mut SERVER_INFO_1523;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1524 = *mut SERVER_INFO_1524;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1525 = *mut SERVER_INFO_1525;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1528 = *mut SERVER_INFO_1528;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1529 = *mut SERVER_INFO_1529;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1530 = *mut SERVER_INFO_1530;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1533 = *mut SERVER_INFO_1533;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1534 = *mut SERVER_INFO_1534;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1535 = *mut SERVER_INFO_1535;
pub type PSERVER_INFO_1536 = *mut SERVER_INFO_1536;
pub type PSERVER_INFO_1537 = *mut SERVER_INFO_1537;
pub type PSERVER_INFO_1538 = *mut SERVER_INFO_1538;
pub type PSERVER_INFO_1539 = *mut SERVER_INFO_1539;
pub type PSERVER_INFO_1540 = *mut SERVER_INFO_1540;
pub type PSERVER_INFO_1541 = *mut SERVER_INFO_1541;
pub type PSERVER_INFO_1542 = *mut SERVER_INFO_1542;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1543 = *mut SERVER_INFO_1543;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1544 = *mut SERVER_INFO_1544;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1545 = *mut SERVER_INFO_1545;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1546 = *mut SERVER_INFO_1546;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1547 = *mut SERVER_INFO_1547;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1548 = *mut SERVER_INFO_1548;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1549 = *mut SERVER_INFO_1549;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1550 = *mut SERVER_INFO_1550;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1552 = *mut SERVER_INFO_1552;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1553 = *mut SERVER_INFO_1553;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1554 = *mut SERVER_INFO_1554;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1555 = *mut SERVER_INFO_1555;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1556 = *mut SERVER_INFO_1556;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1557 = *mut SERVER_INFO_1557;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1560 = *mut SERVER_INFO_1560;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1561 = *mut SERVER_INFO_1561;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1562 = *mut SERVER_INFO_1562;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1563 = *mut SERVER_INFO_1563;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1564 = *mut SERVER_INFO_1564;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1565 = *mut SERVER_INFO_1565;
pub type PSERVER_INFO_1566 = *mut SERVER_INFO_1566;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1567 = *mut SERVER_INFO_1567;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1568 = *mut SERVER_INFO_1568;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1569 = *mut SERVER_INFO_1569;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1570 = *mut SERVER_INFO_1570;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1571 = *mut SERVER_INFO_1571;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1572 = *mut SERVER_INFO_1572;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1573 = *mut SERVER_INFO_1573;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1574 = *mut SERVER_INFO_1574;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1575 = *mut SERVER_INFO_1575;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1576 = *mut SERVER_INFO_1576;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1577 = *mut SERVER_INFO_1577;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1578 = *mut SERVER_INFO_1578;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1579 = *mut SERVER_INFO_1579;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1580 = *mut SERVER_INFO_1580;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1581 = *mut SERVER_INFO_1581;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1582 = *mut SERVER_INFO_1582;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1583 = *mut SERVER_INFO_1583;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1584 = *mut SERVER_INFO_1584;
pub type PSERVER_INFO_1585 = *mut SERVER_INFO_1585;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1586 = *mut SERVER_INFO_1586;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1587 = *mut SERVER_INFO_1587;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1588 = *mut SERVER_INFO_1588;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1590 = *mut SERVER_INFO_1590;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1591 = *mut SERVER_INFO_1591;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1592 = *mut SERVER_INFO_1592;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1593 = *mut SERVER_INFO_1593;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1594 = *mut SERVER_INFO_1594;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1595 = *mut SERVER_INFO_1595;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1596 = *mut SERVER_INFO_1596;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1597 = *mut SERVER_INFO_1597;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1598 = *mut SERVER_INFO_1598;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1599 = *mut SERVER_INFO_1599;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1600 = *mut SERVER_INFO_1600;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_1601 = *mut SERVER_INFO_1601;
pub type PSERVER_INFO_1602 = *mut SERVER_INFO_1602;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_402 = *mut SERVER_INFO_402;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_403 = *mut SERVER_INFO_403;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_502 = *mut SERVER_INFO_502;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_503 = *mut SERVER_INFO_503;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_598 = *mut SERVER_INFO_598;
#[cfg(feature = "shared")]
pub type PSERVER_INFO_599 = *mut SERVER_INFO_599;
#[cfg(feature = "shared")]
pub type PSERVER_TRANSPORT_INFO_0 = *mut SERVER_TRANSPORT_INFO_0;
#[cfg(feature = "shared")]
pub type PSERVER_TRANSPORT_INFO_1 = *mut SERVER_TRANSPORT_INFO_1;
#[cfg(feature = "shared")]
pub type PSERVER_TRANSPORT_INFO_2 = *mut SERVER_TRANSPORT_INFO_2;
#[cfg(feature = "shared")]
pub type PSERVER_TRANSPORT_INFO_3 = *mut SERVER_TRANSPORT_INFO_3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_100 {
    pub sv100_platform_id: super::DWORD,
    pub sv100_name: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1005 {
    pub sv1005_comment: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_101 {
    pub sv101_platform_id: super::DWORD,
    pub sv101_name: windows_core::PWSTR,
    pub sv101_version_major: super::DWORD,
    pub sv101_version_minor: super::DWORD,
    pub sv101_type: super::DWORD,
    pub sv101_comment: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1010 {
    pub sv1010_disc: super::LONG,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1016 {
    pub sv1016_hidden: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1017 {
    pub sv1017_announce: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1018 {
    pub sv1018_anndelta: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_102 {
    pub sv102_platform_id: super::DWORD,
    pub sv102_name: windows_core::PWSTR,
    pub sv102_version_major: super::DWORD,
    pub sv102_version_minor: super::DWORD,
    pub sv102_type: super::DWORD,
    pub sv102_comment: windows_core::PWSTR,
    pub sv102_users: super::DWORD,
    pub sv102_disc: super::LONG,
    pub sv102_hidden: windows_core::BOOL,
    pub sv102_announce: super::DWORD,
    pub sv102_anndelta: super::DWORD,
    pub sv102_licenses: super::DWORD,
    pub sv102_userpath: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_103 {
    pub sv103_platform_id: super::DWORD,
    pub sv103_name: windows_core::PWSTR,
    pub sv103_version_major: super::DWORD,
    pub sv103_version_minor: super::DWORD,
    pub sv103_type: super::DWORD,
    pub sv103_comment: windows_core::PWSTR,
    pub sv103_users: super::DWORD,
    pub sv103_disc: super::LONG,
    pub sv103_hidden: windows_core::BOOL,
    pub sv103_announce: super::DWORD,
    pub sv103_anndelta: super::DWORD,
    pub sv103_licenses: super::DWORD,
    pub sv103_userpath: windows_core::PWSTR,
    pub sv103_capabilities: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1107 {
    pub sv1107_users: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1501 {
    pub sv1501_sessopens: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1502 {
    pub sv1502_sessvcs: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1503 {
    pub sv1503_opensearch: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1506 {
    pub sv1506_maxworkitems: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1509 {
    pub sv1509_maxrawbuflen: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1510 {
    pub sv1510_sessusers: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1511 {
    pub sv1511_sessconns: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1512 {
    pub sv1512_maxnonpagedmemoryusage: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1513 {
    pub sv1513_maxpagedmemoryusage: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1514 {
    pub sv1514_enablesoftcompat: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1515 {
    pub sv1515_enableforcedlogoff: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1516 {
    pub sv1516_timesource: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1518 {
    pub sv1518_lmannounce: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1520 {
    pub sv1520_maxcopyreadlen: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1521 {
    pub sv1521_maxcopywritelen: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1522 {
    pub sv1522_minkeepsearch: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1523 {
    pub sv1523_maxkeepsearch: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1524 {
    pub sv1524_minkeepcomplsearch: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1525 {
    pub sv1525_maxkeepcomplsearch: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1528 {
    pub sv1528_scavtimeout: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1529 {
    pub sv1529_minrcvqueue: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1530 {
    pub sv1530_minfreeworkitems: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1533 {
    pub sv1533_maxmpxct: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1534 {
    pub sv1534_oplockbreakwait: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1535 {
    pub sv1535_oplockbreakresponsewait: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1536 {
    pub sv1536_enableoplocks: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1537 {
    pub sv1537_enableoplockforceclose: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1538 {
    pub sv1538_enablefcbopens: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1539 {
    pub sv1539_enableraw: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1540 {
    pub sv1540_enablesharednetdrives: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1541 {
    pub sv1541_minfreeconnections: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1542 {
    pub sv1542_maxfreeconnections: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1543 {
    pub sv1543_initsesstable: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1544 {
    pub sv1544_initconntable: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1545 {
    pub sv1545_initfiletable: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1546 {
    pub sv1546_initsearchtable: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1547 {
    pub sv1547_alertschedule: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1548 {
    pub sv1548_errorthreshold: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1549 {
    pub sv1549_networkerrorthreshold: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1550 {
    pub sv1550_diskspacethreshold: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1552 {
    pub sv1552_maxlinkdelay: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1553 {
    pub sv1553_minlinkthroughput: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1554 {
    pub sv1554_linkinfovalidtime: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1555 {
    pub sv1555_scavqosinfoupdatetime: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1556 {
    pub sv1556_maxworkitemidletime: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1557 {
    pub sv1557_maxrawworkitems: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1560 {
    pub sv1560_producttype: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1561 {
    pub sv1561_serversize: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1562 {
    pub sv1562_connectionlessautodisc: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1563 {
    pub sv1563_sharingviolationretries: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1564 {
    pub sv1564_sharingviolationdelay: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1565 {
    pub sv1565_maxglobalopensearch: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1566 {
    pub sv1566_removeduplicatesearches: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1567 {
    pub sv1567_lockviolationretries: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1568 {
    pub sv1568_lockviolationoffset: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1569 {
    pub sv1569_lockviolationdelay: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1570 {
    pub sv1570_mdlreadswitchover: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1571 {
    pub sv1571_cachedopenlimit: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1572 {
    pub sv1572_criticalthreads: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1573 {
    pub sv1573_restrictnullsessaccess: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1574 {
    pub sv1574_enablewfw311directipx: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1575 {
    pub sv1575_otherqueueaffinity: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1576 {
    pub sv1576_queuesamplesecs: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1577 {
    pub sv1577_balancecount: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1578 {
    pub sv1578_preferredaffinity: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1579 {
    pub sv1579_maxfreerfcbs: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1580 {
    pub sv1580_maxfreemfcbs: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1581 {
    pub sv1581_maxfreemlcbs: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1582 {
    pub sv1582_maxfreepagedpoolchunks: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1583 {
    pub sv1583_minpagedpoolchunksize: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1584 {
    pub sv1584_maxpagedpoolchunksize: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1585 {
    pub sv1585_sendsfrompreferredprocessor: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1586 {
    pub sv1586_maxthreadsperqueue: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1587 {
    pub sv1587_cacheddirectorylimit: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1588 {
    pub sv1588_maxcopylength: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1590 {
    pub sv1590_enablecompression: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1591 {
    pub sv1591_autosharewks: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1592 {
    pub sv1592_autosharewks: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1593 {
    pub sv1593_enablesecuritysignature: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1594 {
    pub sv1594_requiresecuritysignature: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1595 {
    pub sv1595_minclientbuffersize: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1596 {
    pub sv1596_ConnectionNoSessionsTimeout: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1597 {
    pub sv1597_IdleThreadTimeOut: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1598 {
    pub sv1598_enableW9xsecuritysignature: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1599 {
    pub sv1598_enforcekerberosreauthentication: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1600 {
    pub sv1598_disabledos: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1601 {
    pub sv1598_lowdiskspaceminimum: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_1602 {
    pub sv_1598_disablestrictnamechecking: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_402 {
    pub sv402_ulist_mtime: super::DWORD,
    pub sv402_glist_mtime: super::DWORD,
    pub sv402_alist_mtime: super::DWORD,
    pub sv402_alerts: windows_core::PWSTR,
    pub sv402_security: super::DWORD,
    pub sv402_numadmin: super::DWORD,
    pub sv402_lanmask: super::DWORD,
    pub sv402_guestacct: windows_core::PWSTR,
    pub sv402_chdevs: super::DWORD,
    pub sv402_chdevq: super::DWORD,
    pub sv402_chdevjobs: super::DWORD,
    pub sv402_connections: super::DWORD,
    pub sv402_shares: super::DWORD,
    pub sv402_openfiles: super::DWORD,
    pub sv402_sessopens: super::DWORD,
    pub sv402_sessvcs: super::DWORD,
    pub sv402_sessreqs: super::DWORD,
    pub sv402_opensearch: super::DWORD,
    pub sv402_activelocks: super::DWORD,
    pub sv402_numreqbuf: super::DWORD,
    pub sv402_sizreqbuf: super::DWORD,
    pub sv402_numbigbuf: super::DWORD,
    pub sv402_numfiletasks: super::DWORD,
    pub sv402_alertsched: super::DWORD,
    pub sv402_erroralert: super::DWORD,
    pub sv402_logonalert: super::DWORD,
    pub sv402_accessalert: super::DWORD,
    pub sv402_diskalert: super::DWORD,
    pub sv402_netioalert: super::DWORD,
    pub sv402_maxauditsz: super::DWORD,
    pub sv402_srvheuristics: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_403 {
    pub sv403_ulist_mtime: super::DWORD,
    pub sv403_glist_mtime: super::DWORD,
    pub sv403_alist_mtime: super::DWORD,
    pub sv403_alerts: windows_core::PWSTR,
    pub sv403_security: super::DWORD,
    pub sv403_numadmin: super::DWORD,
    pub sv403_lanmask: super::DWORD,
    pub sv403_guestacct: windows_core::PWSTR,
    pub sv403_chdevs: super::DWORD,
    pub sv403_chdevq: super::DWORD,
    pub sv403_chdevjobs: super::DWORD,
    pub sv403_connections: super::DWORD,
    pub sv403_shares: super::DWORD,
    pub sv403_openfiles: super::DWORD,
    pub sv403_sessopens: super::DWORD,
    pub sv403_sessvcs: super::DWORD,
    pub sv403_sessreqs: super::DWORD,
    pub sv403_opensearch: super::DWORD,
    pub sv403_activelocks: super::DWORD,
    pub sv403_numreqbuf: super::DWORD,
    pub sv403_sizreqbuf: super::DWORD,
    pub sv403_numbigbuf: super::DWORD,
    pub sv403_numfiletasks: super::DWORD,
    pub sv403_alertsched: super::DWORD,
    pub sv403_erroralert: super::DWORD,
    pub sv403_logonalert: super::DWORD,
    pub sv403_accessalert: super::DWORD,
    pub sv403_diskalert: super::DWORD,
    pub sv403_netioalert: super::DWORD,
    pub sv403_maxauditsz: super::DWORD,
    pub sv403_srvheuristics: windows_core::PWSTR,
    pub sv403_auditedevents: super::DWORD,
    pub sv403_autoprofile: super::DWORD,
    pub sv403_autopath: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_502 {
    pub sv502_sessopens: super::DWORD,
    pub sv502_sessvcs: super::DWORD,
    pub sv502_opensearch: super::DWORD,
    pub sv502_sizreqbuf: super::DWORD,
    pub sv502_initworkitems: super::DWORD,
    pub sv502_maxworkitems: super::DWORD,
    pub sv502_rawworkitems: super::DWORD,
    pub sv502_irpstacksize: super::DWORD,
    pub sv502_maxrawbuflen: super::DWORD,
    pub sv502_sessusers: super::DWORD,
    pub sv502_sessconns: super::DWORD,
    pub sv502_maxpagedmemoryusage: super::DWORD,
    pub sv502_maxnonpagedmemoryusage: super::DWORD,
    pub sv502_enablesoftcompat: windows_core::BOOL,
    pub sv502_enableforcedlogoff: windows_core::BOOL,
    pub sv502_timesource: windows_core::BOOL,
    pub sv502_acceptdownlevelapis: windows_core::BOOL,
    pub sv502_lmannounce: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_503 {
    pub sv503_sessopens: super::DWORD,
    pub sv503_sessvcs: super::DWORD,
    pub sv503_opensearch: super::DWORD,
    pub sv503_sizreqbuf: super::DWORD,
    pub sv503_initworkitems: super::DWORD,
    pub sv503_maxworkitems: super::DWORD,
    pub sv503_rawworkitems: super::DWORD,
    pub sv503_irpstacksize: super::DWORD,
    pub sv503_maxrawbuflen: super::DWORD,
    pub sv503_sessusers: super::DWORD,
    pub sv503_sessconns: super::DWORD,
    pub sv503_maxpagedmemoryusage: super::DWORD,
    pub sv503_maxnonpagedmemoryusage: super::DWORD,
    pub sv503_enablesoftcompat: windows_core::BOOL,
    pub sv503_enableforcedlogoff: windows_core::BOOL,
    pub sv503_timesource: windows_core::BOOL,
    pub sv503_acceptdownlevelapis: windows_core::BOOL,
    pub sv503_lmannounce: windows_core::BOOL,
    pub sv503_domain: windows_core::PWSTR,
    pub sv503_maxcopyreadlen: super::DWORD,
    pub sv503_maxcopywritelen: super::DWORD,
    pub sv503_minkeepsearch: super::DWORD,
    pub sv503_maxkeepsearch: super::DWORD,
    pub sv503_minkeepcomplsearch: super::DWORD,
    pub sv503_maxkeepcomplsearch: super::DWORD,
    pub sv503_threadcountadd: super::DWORD,
    pub sv503_numblockthreads: super::DWORD,
    pub sv503_scavtimeout: super::DWORD,
    pub sv503_minrcvqueue: super::DWORD,
    pub sv503_minfreeworkitems: super::DWORD,
    pub sv503_xactmemsize: super::DWORD,
    pub sv503_threadpriority: super::DWORD,
    pub sv503_maxmpxct: super::DWORD,
    pub sv503_oplockbreakwait: super::DWORD,
    pub sv503_oplockbreakresponsewait: super::DWORD,
    pub sv503_enableoplocks: windows_core::BOOL,
    pub sv503_enableoplockforceclose: windows_core::BOOL,
    pub sv503_enablefcbopens: windows_core::BOOL,
    pub sv503_enableraw: windows_core::BOOL,
    pub sv503_enablesharednetdrives: windows_core::BOOL,
    pub sv503_minfreeconnections: super::DWORD,
    pub sv503_maxfreeconnections: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_598 {
    pub sv598_maxrawworkitems: super::DWORD,
    pub sv598_maxthreadsperqueue: super::DWORD,
    pub sv598_producttype: super::DWORD,
    pub sv598_serversize: super::DWORD,
    pub sv598_connectionlessautodisc: super::DWORD,
    pub sv598_sharingviolationretries: super::DWORD,
    pub sv598_sharingviolationdelay: super::DWORD,
    pub sv598_maxglobalopensearch: super::DWORD,
    pub sv598_removeduplicatesearches: super::DWORD,
    pub sv598_lockviolationoffset: super::DWORD,
    pub sv598_lockviolationdelay: super::DWORD,
    pub sv598_mdlreadswitchover: super::DWORD,
    pub sv598_cachedopenlimit: super::DWORD,
    pub sv598_otherqueueaffinity: super::DWORD,
    pub sv598_restrictnullsessaccess: windows_core::BOOL,
    pub sv598_enablewfw311directipx: windows_core::BOOL,
    pub sv598_queuesamplesecs: super::DWORD,
    pub sv598_balancecount: super::DWORD,
    pub sv598_preferredaffinity: super::DWORD,
    pub sv598_maxfreerfcbs: super::DWORD,
    pub sv598_maxfreemfcbs: super::DWORD,
    pub sv598_maxfreelfcbs: super::DWORD,
    pub sv598_maxfreepagedpoolchunks: super::DWORD,
    pub sv598_minpagedpoolchunksize: super::DWORD,
    pub sv598_maxpagedpoolchunksize: super::DWORD,
    pub sv598_sendsfrompreferredprocessor: windows_core::BOOL,
    pub sv598_cacheddirectorylimit: super::DWORD,
    pub sv598_maxcopylength: super::DWORD,
    pub sv598_enablecompression: windows_core::BOOL,
    pub sv598_autosharewks: windows_core::BOOL,
    pub sv598_autoshareserver: windows_core::BOOL,
    pub sv598_enablesecuritysignature: windows_core::BOOL,
    pub sv598_requiresecuritysignature: windows_core::BOOL,
    pub sv598_minclientbuffersize: super::DWORD,
    pub sv598_serverguid: windows_core::GUID,
    pub sv598_ConnectionNoSessionsTimeout: super::DWORD,
    pub sv598_IdleThreadTimeOut: super::DWORD,
    pub sv598_enableW9xsecuritysignature: windows_core::BOOL,
    pub sv598_enforcekerberosreauthentication: windows_core::BOOL,
    pub sv598_disabledos: windows_core::BOOL,
    pub sv598_lowdiskspaceminimum: super::DWORD,
    pub sv598_disablestrictnamechecking: windows_core::BOOL,
    pub sv598_enableauthenticateusersharing: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_INFO_599 {
    pub sv599_sessopens: super::DWORD,
    pub sv599_sessvcs: super::DWORD,
    pub sv599_opensearch: super::DWORD,
    pub sv599_sizreqbuf: super::DWORD,
    pub sv599_initworkitems: super::DWORD,
    pub sv599_maxworkitems: super::DWORD,
    pub sv599_rawworkitems: super::DWORD,
    pub sv599_irpstacksize: super::DWORD,
    pub sv599_maxrawbuflen: super::DWORD,
    pub sv599_sessusers: super::DWORD,
    pub sv599_sessconns: super::DWORD,
    pub sv599_maxpagedmemoryusage: super::DWORD,
    pub sv599_maxnonpagedmemoryusage: super::DWORD,
    pub sv599_enablesoftcompat: windows_core::BOOL,
    pub sv599_enableforcedlogoff: windows_core::BOOL,
    pub sv599_timesource: windows_core::BOOL,
    pub sv599_acceptdownlevelapis: windows_core::BOOL,
    pub sv599_lmannounce: windows_core::BOOL,
    pub sv599_domain: windows_core::PWSTR,
    pub sv599_maxcopyreadlen: super::DWORD,
    pub sv599_maxcopywritelen: super::DWORD,
    pub sv599_minkeepsearch: super::DWORD,
    pub sv599_maxkeepsearch: super::DWORD,
    pub sv599_minkeepcomplsearch: super::DWORD,
    pub sv599_maxkeepcomplsearch: super::DWORD,
    pub sv599_threadcountadd: super::DWORD,
    pub sv599_numblockthreads: super::DWORD,
    pub sv599_scavtimeout: super::DWORD,
    pub sv599_minrcvqueue: super::DWORD,
    pub sv599_minfreeworkitems: super::DWORD,
    pub sv599_xactmemsize: super::DWORD,
    pub sv599_threadpriority: super::DWORD,
    pub sv599_maxmpxct: super::DWORD,
    pub sv599_oplockbreakwait: super::DWORD,
    pub sv599_oplockbreakresponsewait: super::DWORD,
    pub sv599_enableoplocks: windows_core::BOOL,
    pub sv599_enableoplockforceclose: windows_core::BOOL,
    pub sv599_enablefcbopens: windows_core::BOOL,
    pub sv599_enableraw: windows_core::BOOL,
    pub sv599_enablesharednetdrives: windows_core::BOOL,
    pub sv599_minfreeconnections: super::DWORD,
    pub sv599_maxfreeconnections: super::DWORD,
    pub sv599_initsesstable: super::DWORD,
    pub sv599_initconntable: super::DWORD,
    pub sv599_initfiletable: super::DWORD,
    pub sv599_initsearchtable: super::DWORD,
    pub sv599_alertschedule: super::DWORD,
    pub sv599_errorthreshold: super::DWORD,
    pub sv599_networkerrorthreshold: super::DWORD,
    pub sv599_diskspacethreshold: super::DWORD,
    pub sv599_reserved: super::DWORD,
    pub sv599_maxlinkdelay: super::DWORD,
    pub sv599_minlinkthroughput: super::DWORD,
    pub sv599_linkinfovalidtime: super::DWORD,
    pub sv599_scavqosinfoupdatetime: super::DWORD,
    pub sv599_maxworkitemidletime: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_TRANSPORT_INFO_0 {
    pub svti0_numberofvcs: super::DWORD,
    pub svti0_transportname: windows_core::PWSTR,
    pub svti0_transportaddress: super::LPBYTE,
    pub svti0_transportaddresslength: super::DWORD,
    pub svti0_networkaddress: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_TRANSPORT_INFO_1 {
    pub svti1_numberofvcs: super::DWORD,
    pub svti1_transportname: windows_core::PWSTR,
    pub svti1_transportaddress: super::LPBYTE,
    pub svti1_transportaddresslength: super::DWORD,
    pub svti1_networkaddress: windows_core::PWSTR,
    pub svti1_domain: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_TRANSPORT_INFO_2 {
    pub svti2_numberofvcs: super::DWORD,
    pub svti2_transportname: windows_core::PWSTR,
    pub svti2_transportaddress: super::LPBYTE,
    pub svti2_transportaddresslength: super::DWORD,
    pub svti2_networkaddress: windows_core::PWSTR,
    pub svti2_domain: windows_core::PWSTR,
    pub svti2_flags: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_TRANSPORT_INFO_3 {
    pub svti3_numberofvcs: super::DWORD,
    pub svti3_transportname: windows_core::PWSTR,
    pub svti3_transportaddress: super::LPBYTE,
    pub svti3_transportaddresslength: super::DWORD,
    pub svti3_networkaddress: windows_core::PWSTR,
    pub svti3_domain: windows_core::PWSTR,
    pub svti3_flags: super::ULONG,
    pub svti3_passwordlength: super::DWORD,
    pub svti3_password: [super::BYTE; 256],
}
#[cfg(feature = "shared")]
impl Default for SERVER_TRANSPORT_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SRV_HASH_GENERATION_ACTIVE: i32 = 2;
pub const SRV_SUPPORT_HASH_GENERATION: i32 = 1;
pub const SVI1_NUM_ELEMENTS: i32 = 5;
pub const SVI2_NUM_ELEMENTS: i32 = 40;
pub const SVI3_NUM_ELEMENTS: i32 = 44;
pub const SVTI2_CLUSTER_DNN_NAME: i32 = 16;
pub const SVTI2_CLUSTER_NAME: i32 = 8;
pub const SVTI2_REMAP_PIPE_NAMES: i32 = 2;
pub const SVTI2_RESERVED1: i32 = 4096;
pub const SVTI2_RESERVED2: i32 = 8192;
pub const SVTI2_RESERVED3: i32 = 16384;
pub const SVTI2_SCOPED_NAME: i32 = 4;
pub const SVTI2_UNICODE_TRANSPORT_ADDRESS: i32 = 32;
pub const SVTI2_VALID_FLAGS: i32 = 62;
pub const SV_ACCEPTDOWNLEVELAPIS_PARMNUM: i32 = 517;
pub const SV_ACCESSALERT_PARMNUM: i32 = 40;
pub const SV_ACTIVELOCKS_PARMNUM: i32 = 419;
pub const SV_ALERTSCHEDULE_INFOLEVEL: i32 = 1547;
pub const SV_ALERTSCHEDULE_PARMNUM: i32 = 547;
pub const SV_ALERTSCHED_PARMNUM: i32 = 37;
pub const SV_ALERTS_PARMNUM: i32 = 11;
pub const SV_ALIST_MTIME_PARMNUM: i32 = 403;
pub const SV_ANNDELTA_INFOLEVEL: i32 = 1018;
pub const SV_ANNDELTA_PARMNUM: i32 = 18;
pub const SV_ANNOUNCE_INFOLEVEL: i32 = 1017;
pub const SV_ANNOUNCE_PARMNUM: i32 = 17;
pub const SV_AUTOSHARESERVER_INFOLEVEL: i32 = 1592;
pub const SV_AUTOSHARESERVER_PARMNUM: i32 = 592;
pub const SV_AUTOSHAREWKS_INFOLEVEL: i32 = 1591;
pub const SV_AUTOSHAREWKS_PARMNUM: i32 = 591;
pub const SV_BALANCECOUNT_INFOLEVEL: i32 = 1577;
pub const SV_BALANCECOUNT_PARMNUM: i32 = 577;
pub const SV_CACHEDDIRECTORYLIMIT_INFOLEVEL: i32 = 1587;
pub const SV_CACHEDDIRECTORYLIMIT_PARMNUM: i32 = 587;
pub const SV_CACHEDOPENLIMIT_INFOLOEVEL: i32 = 1571;
pub const SV_CACHEDOPENLIMIT_PARMNUM: i32 = 571;
pub const SV_CHDEVJOBS_PARMNUM: i32 = 411;
pub const SV_CHDEVQ_PARMNUM: i32 = 410;
pub const SV_COMMENT_INFOLEVEL: i32 = 1005;
pub const SV_COMMENT_PARMNUM: i32 = 5;
pub const SV_CONNECTIONLESSAUTODISC_INFOLOEVEL: i32 = 1562;
pub const SV_CONNECTIONLESSAUTODISC_PARMNUM: i32 = 562;
pub const SV_CONNECTIONNOSESSIONSTIMEOUT_INFOLEVEL: i32 = 1596;
pub const SV_CONNECTIONNOSESSIONSTIMEOUT_PARMNUM: i32 = 596;
pub const SV_CONNECTIONS_PARMNUM: i32 = 412;
pub const SV_CRITICALTHREADS_INFOLOEVEL: i32 = 1572;
pub const SV_CRITICALTHREADS_PARMNUM: i32 = 572;
pub const SV_DISABLEDOS_INFOLEVEL: i32 = 1600;
pub const SV_DISABLEDOS_PARMNUM: i32 = 600;
pub const SV_DISABLESTRICTNAMECHECKING_INFOLEVEL: i32 = 1602;
pub const SV_DISABLESTRICTNAMECHECKING_PARMNUM: i32 = 602;
pub const SV_DISC_INFOLEVEL: i32 = 1010;
pub const SV_DISC_PARMNUM: i32 = 10;
pub const SV_DISKALERT_PARMNUM: i32 = 41;
pub const SV_DISKSPACETHRESHOLD_INFOLEVEL: i32 = 1550;
pub const SV_DISKSPACETHRESHOLD_PARMNUM: i32 = 550;
pub const SV_DOMAIN_PARMNUM: i32 = 519;
pub const SV_ENABLEAUTHENTICATEUSERSHARING_INFOLEVEL: i32 = 1603;
pub const SV_ENABLEAUTHENTICATEUSERSHARING_PARMNUM: i32 = 603;
pub const SV_ENABLECOMPRESSION_INFOLEVEL: i32 = 1590;
pub const SV_ENABLECOMPRESSION_PARMNUM: i32 = 590;
pub const SV_ENABLEFCBOPENS_INFOLEVEL: i32 = 1538;
pub const SV_ENABLEFCBOPENS_PARMNUM: i32 = 538;
pub const SV_ENABLEFORCEDLOGOFF_INFOLEVEL: i32 = 1515;
pub const SV_ENABLEFORCEDLOGOFF_PARMNUM: i32 = 515;
pub const SV_ENABLEOPLOCKFORCECLOSE_INFOLEVEL: i32 = 1537;
pub const SV_ENABLEOPLOCKFORCECLOSE_PARMNUM: i32 = 537;
pub const SV_ENABLEOPLOCKS_INFOLEVEL: i32 = 1536;
pub const SV_ENABLEOPLOCKS_PARMNUM: i32 = 536;
pub const SV_ENABLERAW_INFOLEVEL: i32 = 1539;
pub const SV_ENABLERAW_PARMNUM: i32 = 539;
pub const SV_ENABLESECURITYSIGNATURE_INFOLEVEL: i32 = 1593;
pub const SV_ENABLESECURITYSIGNATURE_PARMNUM: i32 = 593;
pub const SV_ENABLESHAREDNETDRIVES_INFOLEVEL: i32 = 1540;
pub const SV_ENABLESHAREDNETDRIVES_PARMNUM: i32 = 540;
pub const SV_ENABLESOFTCOMPAT_INFOLEVEL: i32 = 1514;
pub const SV_ENABLESOFTCOMPAT_PARMNUM: i32 = 514;
pub const SV_ENABLEW9XSECURITYSIGNATURE_INFOLEVEL: i32 = 1598;
pub const SV_ENABLEW9XSECURITYSIGNATURE_PARMNUM: i32 = 598;
pub const SV_ENABLEWFW311DIRECTIPX_INFOLOEVEL: i32 = 1574;
pub const SV_ENABLEWFW311DIRECTIPX_PARMNUM: i32 = 574;
pub const SV_ENFORCEKERBEROSREAUTHENTICATION_INFOLEVEL: i32 = 1599;
pub const SV_ENFORCEKERBEROSREAUTHENTICATION_PARMNUM: i32 = 599;
pub const SV_ERRORALERT_PARMNUM: i32 = 38;
pub const SV_ERRORTHRESHOLD_INFOLEVEL: i32 = 1548;
pub const SV_ERRORTHRESHOLD_PARMNUM: i32 = 548;
pub const SV_GLIST_MTIME_PARMNUM: i32 = 402;
pub const SV_GUESTACC_PARMNUM: i32 = 408;
pub const SV_HIDDEN: i32 = 1;
pub const SV_HIDDEN_INFOLEVEL: i32 = 1016;
pub const SV_HIDDEN_PARMNUM: i32 = 16;
pub const SV_IDLETHREADTIMEOUT_INFOLEVEL: i32 = 1597;
pub const SV_IDLETHREADTIMEOUT_PARMNUM: i32 = 597;
pub const SV_INITCONNTABLE_INFOLEVEL: i32 = 1544;
pub const SV_INITCONNTABLE_PARMNUM: i32 = 544;
pub const SV_INITFILETABLE_INFOLEVEL: i32 = 1545;
pub const SV_INITFILETABLE_PARMNUM: i32 = 545;
pub const SV_INITSEARCHTABLE_INFOLEVEL: i32 = 1546;
pub const SV_INITSEARCHTABLE_PARMNUM: i32 = 546;
pub const SV_INITSESSTABLE_INFOLEVEL: i32 = 1543;
pub const SV_INITSESSTABLE_PARMNUM: i32 = 543;
pub const SV_INITWORKITEMS_PARMNUM: i32 = 505;
pub const SV_IRPSTACKSIZE_PARMNUM: i32 = 508;
pub const SV_LANMASK_PARMNUM: i32 = 407;
pub const SV_LINKINFOVALIDTIME_INFOLEVEL: i32 = 1554;
pub const SV_LINKINFOVALIDTIME_PARMNUM: i32 = 554;
pub const SV_LMANNOUNCE_INFOLEVEL: i32 = 1518;
pub const SV_LMANNOUNCE_PARMNUM: i32 = 518;
pub const SV_LOCKVIOLATIONDELAY_INFOLOEVEL: i32 = 1569;
pub const SV_LOCKVIOLATIONDELAY_PARMNUM: i32 = 569;
pub const SV_LOCKVIOLATIONOFFSET_INFOLOEVEL: i32 = 1568;
pub const SV_LOCKVIOLATIONOFFSET_PARMNUM: i32 = 568;
pub const SV_LOCKVIOLATIONRETRIES_INFOLOEVEL: i32 = 1567;
pub const SV_LOCKVIOLATIONRETRIES_PARMNUM: i32 = 567;
pub const SV_LOGONALERT_PARMNUM: i32 = 39;
pub const SV_LOWDISKSPACEMINIMUM_INFOLEVEL: i32 = 1601;
pub const SV_LOWDISKSPACEMINIMUM_PARMNUM: i32 = 601;
pub const SV_MAXAUDITSZ_PARMNUM: i32 = 43;
pub const SV_MAXCOPYLENGTH_INFOLEVEL: i32 = 1588;
pub const SV_MAXCOPYLENGTH_PARMNUM: i32 = 588;
pub const SV_MAXCOPYREADLEN_INFOLEVEL: i32 = 1520;
pub const SV_MAXCOPYREADLEN_PARMNUM: i32 = 520;
pub const SV_MAXCOPYWRITELEN_INFOLEVEL: i32 = 1521;
pub const SV_MAXCOPYWRITELEN_PARMNUM: i32 = 521;
pub const SV_MAXFREECONNECTIONS_INFOLEVEL: i32 = 1542;
pub const SV_MAXFREECONNECTIONS_PARMNUM: i32 = 542;
pub const SV_MAXFREELFCBS_INFOLEVEL: i32 = 1581;
pub const SV_MAXFREELFCBS_PARMNUM: i32 = 581;
pub const SV_MAXFREEMFCBS_INFOLEVEL: i32 = 1580;
pub const SV_MAXFREEMFCBS_PARMNUM: i32 = 580;
pub const SV_MAXFREEPAGEDPOOLCHUNKS_INFOLEVEL: i32 = 1582;
pub const SV_MAXFREEPAGEDPOOLCHUNKS_PARMNUM: i32 = 582;
pub const SV_MAXFREERFCBS_INFOLEVEL: i32 = 1579;
pub const SV_MAXFREERFCBS_PARMNUM: i32 = 579;
pub const SV_MAXGLOBALOPENSEARCH_INFOLOEVEL: i32 = 1565;
pub const SV_MAXGLOBALOPENSEARCH_PARMNUM: i32 = 565;
pub const SV_MAXKEEPCOMPLSEARCH_INFOLEVEL: i32 = 1525;
pub const SV_MAXKEEPCOMPLSEARCH_PARMNUM: i32 = 525;
pub const SV_MAXKEEPSEARCH_INFOLEVEL: i32 = 1523;
pub const SV_MAXKEEPSEARCH_PARMNUM: i32 = 523;
pub const SV_MAXLINKDELAY_INFOLEVEL: i32 = 1552;
pub const SV_MAXLINKDELAY_PARMNUM: i32 = 552;
pub const SV_MAXMPXCT_INFOLEVEL: i32 = 1533;
pub const SV_MAXMPXCT_PARMNUM: i32 = 533;
pub const SV_MAXNONPAGEDMEMORYUSAGE_INFOLEVEL: i32 = 1512;
pub const SV_MAXNONPAGEDMEMORYUSAGE_PARMNUM: i32 = 512;
pub const SV_MAXPAGEDMEMORYUSAGE_INFOLEVEL: i32 = 1513;
pub const SV_MAXPAGEDMEMORYUSAGE_PARMNUM: i32 = 513;
pub const SV_MAXPAGEDPOOLCHUNKSIZE_INFOLEVEL: i32 = 1584;
pub const SV_MAXPAGEDPOOLCHUNKSIZE_PARMNUM: i32 = 584;
pub const SV_MAXRAWBUFLEN_INFOLEVEL: i32 = 1509;
pub const SV_MAXRAWBUFLEN_PARMNUM: i32 = 509;
pub const SV_MAXRAWWORKITEMS_INFOLOEVEL: i32 = 1557;
pub const SV_MAXRAWWORKITEMS_PARMNUM: i32 = 557;
pub const SV_MAXTHREADSPERQUEUE_INFOLEVEL: i32 = 1586;
pub const SV_MAXTHREADSPERQUEUE_PARMNUM: i32 = 586;
pub const SV_MAXWORKITEMIDLETIME_INFOLEVEL: i32 = 1556;
pub const SV_MAXWORKITEMIDLETIME_PARMNUM: i32 = 556;
pub const SV_MAXWORKITEMS_INFOLEVEL: i32 = 1506;
pub const SV_MAXWORKITEMS_PARMNUM: i32 = 506;
pub const SV_MAX_CMD_LEN: i32 = 256;
pub const SV_MAX_SRV_HEUR_LEN: i32 = 32;
pub const SV_MDLREADSWITCHOVER_INFOLOEVEL: i32 = 1570;
pub const SV_MDLREADSWITCHOVER_PARMNUM: i32 = 570;
pub const SV_MINCLIENTBUFFERSIZE_INFOLEVEL: i32 = 1595;
pub const SV_MINCLIENTBUFFERSIZE_PARMNUM: i32 = 595;
pub const SV_MINFREECONNECTIONS_INFOLEVEL: i32 = 1541;
pub const SV_MINFREECONNECTIONS_PARMNUM: i32 = 541;
pub const SV_MINFREEWORKITEMS_INFOLEVEL: i32 = 1530;
pub const SV_MINFREEWORKITEMS_PARMNUM: i32 = 530;
pub const SV_MINKEEPCOMPLSEARCH_INFOLEVEL: i32 = 1524;
pub const SV_MINKEEPCOMPLSEARCH_PARMNUM: i32 = 524;
pub const SV_MINKEEPSEARCH_INFOLEVEL: i32 = 1522;
pub const SV_MINKEEPSEARCH_PARMNUM: i32 = 522;
pub const SV_MINLINKTHROUGHPUT_INFOLEVEL: i32 = 1553;
pub const SV_MINLINKTHROUGHPUT_PARMNUM: i32 = 553;
pub const SV_MINPAGEDPOOLCHUNKSIZE_INFOLEVEL: i32 = 1583;
pub const SV_MINPAGEDPOOLCHUNKSIZE_PARMNUM: i32 = 583;
pub const SV_MINRCVQUEUE_INFOLEVEL: i32 = 1529;
pub const SV_MINRCVQUEUE_PARMNUM: i32 = 529;
pub const SV_NAME_PARMNUM: i32 = 102;
pub const SV_NETIOALERT_PARMNUM: i32 = 42;
pub const SV_NETWORKERRORTHRESHOLD_INFOLEVEL: i32 = 1549;
pub const SV_NETWORKERRORTHRESHOLD_PARMNUM: i32 = 549;
pub const SV_NODISC: i32 = -1;
pub const SV_NUMADMIN_PARMNUM: i32 = 406;
pub const SV_NUMBIGBUF_PARMNUM: i32 = 422;
pub const SV_NUMBLOCKTHREADS_PARMNUM: i32 = 527;
pub const SV_NUMFILETASKS_PARMNUM: i32 = 423;
pub const SV_NUMREQBUF_PARMNUM: i32 = 420;
pub const SV_OPENFILES_PARMNUM: i32 = 414;
pub const SV_OPENSEARCH_INFOLEVEL: i32 = 1503;
pub const SV_OPENSEARCH_PARMNUM: i32 = 503;
pub const SV_OPLOCKBREAKRESPONSEWAIT_INFOLEVEL: i32 = 1535;
pub const SV_OPLOCKBREAKRESPONSEWAIT_PARMNUM: i32 = 535;
pub const SV_OPLOCKBREAKWAIT_INFOLEVEL: i32 = 1534;
pub const SV_OPLOCKBREAKWAIT_PARMNUM: i32 = 534;
pub const SV_OTHERQUEUEAFFINITY_INFOLEVEL: i32 = 1575;
pub const SV_OTHERQUEUEAFFINITY_PARMNUM: i32 = 575;
pub const SV_PLATFORM_ID_NT: i32 = 500;
pub const SV_PLATFORM_ID_OS2: i32 = 400;
pub const SV_PLATFORM_ID_PARMNUM: i32 = 101;
pub const SV_PREFERREDAFFINITY_INFOLEVEL: i32 = 1578;
pub const SV_PREFERREDAFFINITY_PARMNUM: i32 = 578;
pub const SV_PRODUCTTYPE_INFOLOEVEL: i32 = 1560;
pub const SV_PRODUCTTYPE_PARMNUM: i32 = 560;
pub const SV_QUEUESAMPLESECS_INFOLEVEL: i32 = 1576;
pub const SV_QUEUESAMPLESECS_PARMNUM: i32 = 576;
pub const SV_RAWWORKITEMS_PARMNUM: i32 = 507;
pub const SV_REMOVEDUPLICATESEARCHES_INFOLOEVEL: i32 = 1566;
pub const SV_REMOVEDUPLICATESEARCHES_PARMNUM: i32 = 566;
pub const SV_REQUIRESECURITYSIGNATURE_INFOLEVEL: i32 = 1594;
pub const SV_REQUIRESECURITYSIGNATURE_PARMNUM: i32 = 594;
pub const SV_RESTRICTNULLSESSACCESS_INFOLOEVEL: i32 = 1573;
pub const SV_RESTRICTNULLSESSACCESS_PARMNUM: i32 = 573;
pub const SV_SCAVQOSINFOUPDATETIME_INFOLEVEL: i32 = 1555;
pub const SV_SCAVQOSINFOUPDATETIME_PARMNUM: i32 = 555;
pub const SV_SCAVTIMEOUT_INFOLEVEL: i32 = 1528;
pub const SV_SCAVTIMEOUT_PARMNUM: i32 = 528;
pub const SV_SECURITY_PARMNUM: i32 = 405;
pub const SV_SENDSFROMPREFERREDPROCESSOR_INFOLEVEL: i32 = 1585;
pub const SV_SENDSFROMPREFERREDPROCESSOR_PARMNUM: i32 = 585;
pub const SV_SERVERSIZE_INFOLOEVEL: i32 = 1561;
pub const SV_SERVERSIZE_PARMNUM: i32 = 561;
pub const SV_SESSCONNS_INFOLEVEL: i32 = 1511;
pub const SV_SESSCONNS_PARMNUM: i32 = 511;
pub const SV_SESSOPENS_INFOLEVEL: i32 = 1501;
pub const SV_SESSOPENS_PARMNUM: i32 = 501;
pub const SV_SESSREQS_PARMNUM: i32 = 417;
pub const SV_SESSUSERS_INFOLEVEL: i32 = 1510;
pub const SV_SESSUSERS_PARMNUM: i32 = 510;
pub const SV_SESSVCS_INFOLEVEL: i32 = 1502;
pub const SV_SESSVCS_PARMNUM: i32 = 502;
pub const SV_SHARESECURITY: i32 = 0;
pub const SV_SHARES_PARMNUM: i32 = 413;
pub const SV_SHARINGVIOLATIONDELAY_INFOLOEVEL: i32 = 1564;
pub const SV_SHARINGVIOLATIONDELAY_PARMNUM: i32 = 564;
pub const SV_SHARINGVIOLATIONRETRIES_INFOLOEVEL: i32 = 1563;
pub const SV_SHARINGVIOLATIONRETRIES_PARMNUM: i32 = 563;
pub const SV_SIZREQBUF_PARMNUM: i32 = 504;
pub const SV_SRVHEURISTICS_PARMNUM: i32 = 431;
pub const SV_THREADCOUNTADD_PARMNUM: i32 = 526;
pub const SV_THREADPRIORITY_PARMNUM: i32 = 532;
pub const SV_TIMESOURCE_INFOLEVEL: i32 = 1516;
pub const SV_TIMESOURCE_PARMNUM: i32 = 516;
pub const SV_TYPE_AFP: i32 = 64;
pub const SV_TYPE_ALL: u32 = 4294967295;
pub const SV_TYPE_ALTERNATE_XPORT: i32 = 536870912;
pub const SV_TYPE_BACKUP_BROWSER: i32 = 131072;
pub const SV_TYPE_CLUSTER_NT: i32 = 16777216;
pub const SV_TYPE_CLUSTER_VS_NT: i32 = 67108864;
pub const SV_TYPE_DCE: i32 = 268435456;
pub const SV_TYPE_DFS: i32 = 8388608;
pub const SV_TYPE_DIALIN_SERVER: i32 = 1024;
pub const SV_TYPE_DOMAIN_BAKCTRL: i32 = 16;
pub const SV_TYPE_DOMAIN_CTRL: i32 = 8;
pub const SV_TYPE_DOMAIN_ENUM: u32 = 2147483648;
pub const SV_TYPE_DOMAIN_MASTER: i32 = 524288;
pub const SV_TYPE_DOMAIN_MEMBER: i32 = 256;
pub const SV_TYPE_LOCAL_LIST_ONLY: i32 = 1073741824;
pub const SV_TYPE_MASTER_BROWSER: i32 = 262144;
pub const SV_TYPE_NOVELL: i32 = 128;
pub const SV_TYPE_NT: i32 = 4096;
pub const SV_TYPE_PARMNUM: i32 = 105;
pub const SV_TYPE_POTENTIAL_BROWSER: i32 = 65536;
pub const SV_TYPE_PRINTQ_SERVER: i32 = 512;
pub const SV_TYPE_SERVER: i32 = 2;
pub const SV_TYPE_SERVER_MFPN: i32 = 16384;
pub const SV_TYPE_SERVER_NT: i32 = 32768;
pub const SV_TYPE_SERVER_OSF: i32 = 1048576;
pub const SV_TYPE_SERVER_UNIX: i32 = 2048;
pub const SV_TYPE_SERVER_VMS: i32 = 2097152;
pub const SV_TYPE_SQLSERVER: i32 = 4;
pub const SV_TYPE_TERMINALSERVER: i32 = 33554432;
pub const SV_TYPE_TIME_SOURCE: i32 = 32;
pub const SV_TYPE_WFW: i32 = 8192;
pub const SV_TYPE_WINDOWS: i32 = 4194304;
pub const SV_TYPE_WORKSTATION: i32 = 1;
pub const SV_TYPE_XENIX_SERVER: i32 = 2048;
pub const SV_ULIST_MTIME_PARMNUM: i32 = 401;
pub const SV_USERPATH_PARMNUM: i32 = 112;
pub const SV_USERSECURITY: i32 = 1;
pub const SV_USERS_INFOLEVEL: i32 = 1107;
pub const SV_USERS_PARMNUM: i32 = 107;
pub const SV_USERS_PER_LICENSE: i32 = 5;
pub const SV_VERSION_MAJOR_PARMNUM: i32 = 103;
pub const SV_VERSION_MINOR_PARMNUM: i32 = 104;
pub const SV_VISIBLE: i32 = 0;
pub const SV_XACTMEMSIZE_PARMNUM: i32 = 531;
pub const SW_AUTOPROF_LOAD_MASK: i32 = 1;
pub const SW_AUTOPROF_SAVE_MASK: i32 = 2;
