void __stdcall CreateTable(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall MgmAddGroupMembershipEntry(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall MgmDeRegisterMProtocol(int p0) {}
void __stdcall MgmDeleteGroupMembershipEntry(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall MgmGetFirstMfe(int p0, int p1, int p2) {}
void __stdcall MgmGetFirstMfeStats(int p0, int p1, int p2, int p3) {}
void __stdcall MgmGetMfe(int p0, int p1, int p2) {}
void __stdcall MgmGetMfeStats(int p0, int p1, int p2, int p3) {}
void __stdcall MgmGetNextMfe(int p0, int p1, int p2, int p3) {}
void __stdcall MgmGetNextMfeStats(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall MgmGetProtocolOnInterface(int p0, int p1, int p2, int p3) {}
void __stdcall MgmGroupEnumerationEnd(int p0) {}
void __stdcall MgmGroupEnumerationGetNext(int p0, int p1, int p2, int p3) {}
void __stdcall MgmGroupEnumerationStart(int p0, int p1, int p2) {}
void __stdcall MgmRegisterMProtocol(int p0, int p1, int p2, int p3) {}
void __stdcall MgmReleaseInterfaceOwnership(int p0, int p1, int p2) {}
void __stdcall MgmTakeInterfaceOwnership(int p0, int p1, int p2) {}
void __stdcall RtmAddNextHop(int p0, int p1, int p2, int p3) {}
void __stdcall RtmAddRouteToDest(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall RtmBlockMethods(int p0, int p1, int p2, int p3) {}
void __stdcall RtmConvertIpv6AddressAndLengthToNetAddress(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall RtmConvertNetAddressToIpv6AddressAndLength(int p0, int p1, int p2, int p3) {}
void __stdcall RtmCreateDestEnum(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtmCreateNextHopEnum(int p0, int p1, int p2, int p3) {}
void __stdcall RtmCreateRouteEnum(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall RtmCreateRouteList(int p0, int p1) {}
void __stdcall RtmCreateRouteListEnum(int p0, int p1, int p2) {}
void __stdcall RtmDeleteEnumHandle(int p0, int p1) {}
void __stdcall RtmDeleteNextHop(int p0, int p1, int p2) {}
void __stdcall RtmDeleteRouteList(int p0, int p1) {}
void __stdcall RtmDeleteRouteToDest(int p0, int p1, int p2) {}
void __stdcall RtmDeregisterEntity(int p0) {}
void __stdcall RtmDeregisterFromChangeNotification(int p0, int p1) {}
void __stdcall RtmFindNextHop(int p0, int p1, int p2, int p3) {}
void __stdcall RtmGetChangeStatus(int p0, int p1, int p2, int p3) {}
void __stdcall RtmGetChangedDests(int p0, int p1, int p2, int p3) {}
void __stdcall RtmGetDestInfo(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtmGetEntityInfo(int p0, int p1, int p2) {}
void __stdcall RtmGetEntityMethods(int p0, int p1, int p2, int p3) {}
void __stdcall RtmGetEnumDests(int p0, int p1, int p2, int p3) {}
void __stdcall RtmGetEnumNextHops(int p0, int p1, int p2, int p3) {}
void __stdcall RtmGetEnumRoutes(int p0, int p1, int p2, int p3) {}
void __stdcall RtmGetExactMatchDestination(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtmGetExactMatchRoute(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall RtmGetLessSpecificDestination(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtmGetListEnumRoutes(int p0, int p1, int p2, int p3) {}
void __stdcall RtmGetMostSpecificDestination(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtmGetNextHopInfo(int p0, int p1, int p2) {}
void __stdcall RtmGetNextHopPointer(int p0, int p1, int p2) {}
void __stdcall RtmGetOpaqueInformationPointer(int p0, int p1, int p2) {}
void __stdcall RtmGetRegisteredEntities(int p0, int p1, int p2, int p3) {}
void __stdcall RtmGetRouteInfo(int p0, int p1, int p2, int p3) {}
void __stdcall RtmGetRoutePointer(int p0, int p1, int p2) {}
void __stdcall RtmHoldDestination(int p0, int p1, int p2, int p3) {}
void __stdcall RtmIgnoreChangedDests(int p0, int p1, int p2, int p3) {}
void __stdcall RtmInsertInRouteList(int p0, int p1, int p2, int p3) {}
void __stdcall RtmInvokeMethod(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtmIsBestRoute(int p0, int p1, int p2) {}
void __stdcall RtmIsMarkedForChangeNotification(int p0, int p1, int p2, int p3) {}
void __stdcall RtmLockDestination(int p0, int p1, int p2, int p3) {}
void __stdcall RtmLockNextHop(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtmLockRoute(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtmMarkDestForChangeNotification(int p0, int p1, int p2, int p3) {}
void __stdcall RtmReferenceHandles(int p0, int p1, int p2) {}
void __stdcall RtmRegisterEntity(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtmRegisterForChangeNotification(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtmReleaseChangedDests(int p0, int p1, int p2, int p3) {}
void __stdcall RtmReleaseDestInfo(int p0, int p1) {}
void __stdcall RtmReleaseDests(int p0, int p1, int p2) {}
void __stdcall RtmReleaseEntities(int p0, int p1, int p2) {}
void __stdcall RtmReleaseEntityInfo(int p0, int p1) {}
void __stdcall RtmReleaseNextHopInfo(int p0, int p1) {}
void __stdcall RtmReleaseNextHops(int p0, int p1, int p2) {}
void __stdcall RtmReleaseRouteInfo(int p0, int p1) {}
void __stdcall RtmReleaseRoutes(int p0, int p1, int p2) {}
void __stdcall RtmUpdateAndUnlockRoute(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
