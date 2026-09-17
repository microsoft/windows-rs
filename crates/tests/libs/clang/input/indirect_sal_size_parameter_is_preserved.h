//! namespace Sal
//! library api.dll

#define WRITES_BYTES(c) __attribute__((annotate("_Out_writes_bytes_(" #c ")")))
             void Read(unsigned* size, WRITES_BYTES(*size) void* data);
