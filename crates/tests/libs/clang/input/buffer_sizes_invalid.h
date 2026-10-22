#include <sal.h>

#if CASE == 1
typedef void (__stdcall *Invalid)(
    _Out_writes_bytes_(first) _Out_writes_bytes_(second) void* buffer,
    unsigned int first, unsigned int second);
#elif CASE == 2
typedef void (__stdcall *Invalid)(
    _Out_writes_bytes_(capacity) void* buffer, float capacity);
#elif CASE == 3
typedef void (__stdcall *Invalid)(
    _Out_writes_bytes_(capacity) unsigned int buffer, unsigned int capacity);
#elif CASE == 4
typedef void (__stdcall *Invalid)(
    _Out_writes_bytes_(capacity) _Out_writes_(capacity) unsigned short* buffer,
    unsigned int capacity);
#elif CASE == 5
typedef void (*Invalid)(_Out_writes_(capacity + 1) unsigned char* buffer, float capacity);
#elif CASE == 6
typedef void (*Invalid)(_Out_writes_(32) unsigned int buffer);
#endif

typedef void (__stdcall *Supported)(
    _Out_writes_bytes_(capacity) void* buffer, unsigned int capacity);
