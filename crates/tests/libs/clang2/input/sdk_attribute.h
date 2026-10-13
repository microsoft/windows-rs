#include <windows.h>

struct GuidHolder {
    struct _GUID value;
    void* context;
};
