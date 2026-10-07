#include "wdk_layout.h"
#include <stddef.h>

unsigned long WdkLayout(unsigned long index) {
    switch (index) {
    case 0: return sizeof(DeviceIoControl);
    case 1: return alignof(DeviceIoControl);
    case 2: return offsetof(DeviceIoControl, OutputBufferLength);
    case 3: return offsetof(DeviceIoControl, InputBufferLength);
    case 4: return offsetof(DeviceIoControl, IoControlCode);
    case 5: return offsetof(DeviceIoControl, Type3InputBuffer);
    case 6: return sizeof(QuerySecurity);
    case 7: return alignof(QuerySecurity);
    case 8: return offsetof(QuerySecurity, SecurityInformation);
    case 9: return offsetof(QuerySecurity, Length);
    default: return 0xffffffff;
    }
}

void WdkMutate(DeviceIoControl* device, QuerySecurity* security) {
    device->OutputBufferLength += device->InputBufferLength;
    device->InputBufferLength ^= device->IoControlCode;
    device->IoControlCode += security->SecurityInformation;
    security->Length += device->OutputBufferLength;
    security->SecurityInformation ^= 0x13579bdf;
    device->Type3InputBuffer = security;
}
