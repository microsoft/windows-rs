#define _In_reads_(n) __attribute__((annotate("_In_reads_(" #n ")")))
#define _Out_writes_(n) __attribute__((annotate("_Out_writes_(" #n ")")))
#define _Inout_updates_(n) __attribute__((annotate("_Inout_updates_(" #n ")")))
#define _In_reads_bytes_(n) __attribute__((annotate("_In_reads_bytes_(" #n ")")))
#define _Out_writes_bytes_(n) __attribute__((annotate("_Out_writes_bytes_(" #n ")")))
#define _Inout_updates_bytes_(n) __attribute__((annotate("_Inout_updates_bytes_(" #n ")")))

extern "C" void Buffers(
    unsigned count,
    _In_reads_(count) const int* input,
    _Out_writes_(count) int* output,
    _Inout_updates_(count) int* update,
    _In_reads_bytes_(count) const void* input_bytes,
    _Out_writes_bytes_(count) void* output_bytes,
    _Inout_updates_bytes_(count) void* update_bytes,
    _In_reads_(4) const int* fixed);

extern "C" void Zero(_In_reads_(0) const int* data);
extern "C" void Max(_In_reads_(2147483647) const int* data);

struct __declspec(uuid("00000002-0000-0000-c000-000000000046")) IBuffers {
    virtual void __stdcall Buffers(
        _In_reads_(count) const int* input,
        _Out_writes_(count) int* output,
        _Inout_updates_(count) int* update,
        _In_reads_bytes_(count) const void* input_bytes,
        _Out_writes_bytes_(count) void* output_bytes,
        _Inout_updates_bytes_(count) void* update_bytes,
        _In_reads_(4) const int* fixed,
        unsigned count) = 0;
};
