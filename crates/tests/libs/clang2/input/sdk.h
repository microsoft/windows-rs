#include <windows.h>

struct ListHolder {
    struct _LIST_ENTRY value;
    void* context;
};
