#if defined(__clang__)
#define _In_ __attribute__((annotate("_In_")))
#define _In_opt_ __attribute__((annotate("_In_opt_")))
#define _Out_ __attribute__((annotate("_Out_")))
#define _Out_opt_ __attribute__((annotate("_Out_opt_")))
#define _Inout_opt_ __attribute__((annotate("_Inout_opt_")))
#define _Out_writes_bytes_to_(n, c) __attribute__((annotate("_Out_writes_bytes_to_(" #n "," #c ")")))
#define _Out_writes_bytes_to_opt_(n, c) __attribute__((annotate("_Out_writes_bytes_to_opt_(" #n "," #c ")")))
#else
#define _In_
#define _In_opt_
#define _Out_
#define _Out_opt_
#define _Inout_opt_
#define _Out_writes_bytes_to_(n, c)
#define _Out_writes_bytes_to_opt_(n, c)
#endif

extern "C" void FillRequired(
    _Out_writes_bytes_to_(capacity, *written) unsigned char* buffer,
    unsigned capacity, _Out_opt_ unsigned* written);
extern "C" void FillOptional(
    _Out_writes_bytes_to_opt_(capacity, *written) unsigned char* buffer,
    unsigned capacity, _Out_opt_ unsigned* written);
extern "C" void FillInout(
    _Out_writes_bytes_to_(capacity, *written) unsigned char* buffer,
    unsigned capacity, _Inout_opt_ unsigned* written);

typedef const unsigned ReadOnlyCount;
extern "C" void ReadCount(_In_ ReadOnlyCount* count);

typedef void (__cdecl *OptionalCallback)(
    _Out_writes_bytes_to_(capacity, *written) unsigned char* buffer,
    unsigned capacity, _Out_opt_ unsigned* written);
extern "C" void InvokeOptional(OptionalCallback callback,
    _Out_writes_bytes_to_(capacity, *written) unsigned char* buffer,
    unsigned capacity, _Out_opt_ unsigned* written);

struct __declspec(uuid("14610fde-d2d0-4fe6-bd47-2ea5bc3d59b9")) IOptionalOutput {
    virtual void __stdcall Fill(
        _Out_writes_bytes_to_(capacity, *written) unsigned char* buffer,
        unsigned capacity, _Out_opt_ unsigned* written) = 0;
};
