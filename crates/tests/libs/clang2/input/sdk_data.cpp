#include <guiddef.h>
#include <cguid.h>
#include <initguid.h>
#include "sdk_data.h"
#include <cstring>

extern "C" int CheckSdkValues(const void* property, const void* device,
    const void* folder, const void* media, const void* avi, const void* file, const void* network) {
    return std::memcmp(property, &PKEY_Address_Country, sizeof(PROPERTYKEY)) == 0
        && std::memcmp(device, &DEVPKEY_Device_ClassGuid, sizeof(DEVPROPKEY)) == 0
        && std::memcmp(folder, &FOLDERID_Documents, sizeof(GUID)) == 0
        && std::memcmp(media, &MFVideoFormat_RGB32, sizeof(GUID)) == 0
        && std::memcmp(avi, &IID_IAVIFile, sizeof(GUID)) == 0
        && std::memcmp(file, &FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE, sizeof(GUID)) == 0
        && std::memcmp(network, &NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID, sizeof(GUID)) == 0;
}
