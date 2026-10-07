#define DEFINE_VALUES
#include "guid_constants.h"
#include <cstring>

extern "C" int CheckConstants(const void* id, unsigned long id_size,
    const void* key, unsigned long key_size) {
    return id_size == sizeof(ID) && key_size == sizeof(KEY)
        && std::memcmp(id, &ID, sizeof(ID)) == 0
        && std::memcmp(key, &KEY, sizeof(KEY)) == 0;
}
