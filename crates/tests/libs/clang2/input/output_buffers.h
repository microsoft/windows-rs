#define _In_ __attribute__((annotate("_In_")))
#define _Out_ __attribute__((annotate("_Out_")))
#define _Out_opt_ __attribute__((annotate("_Out_opt_")))
#define _Out_writes_bytes_(n) __attribute__((annotate("_Out_writes_bytes_(" #n ")")))
#define _Out_writes_bytes_all_(n) __attribute__((annotate("_Out_writes_bytes_all_(" #n ")")))
#define _Out_writes_bytes_all_opt_(n) __attribute__((annotate("_Out_writes_bytes_all_opt_(" #n ")")))
#define _Out_writes_bytes_to_(n, c) __attribute__((annotate("_Out_writes_bytes_to_(" #n "," #c ")")))
#define _Out_writes_bytes_to_opt_(n, c) __attribute__((annotate("_Out_writes_bytes_to_opt_(" #n "," #c ")")))

extern "C" void Capacity(_Out_writes_bytes_(capacity) unsigned char* buffer, unsigned capacity);
extern "C" void All(_Out_writes_bytes_all_(capacity) unsigned char* buffer, unsigned capacity);
extern "C" void AllOptional(_Out_writes_bytes_all_opt_(capacity) unsigned char* buffer, unsigned capacity);
extern "C" void Partial(_Out_writes_bytes_to_(capacity, *written) unsigned char* buffer,
    unsigned capacity, _Out_ unsigned* written);
extern "C" void PartialOptional(_Out_writes_bytes_to_opt_(capacity, *written) unsigned char* buffer,
    unsigned capacity, _Out_ unsigned* written);
extern "C" void ValueCount(_Out_writes_bytes_to_(capacity, written) unsigned char* buffer,
    unsigned capacity, unsigned written);

struct __declspec(uuid("24610fde-d2d0-4fe6-bd47-2ea5bc3d59b9")) IOutput {
    virtual void __stdcall Partial(
        _Out_writes_bytes_to_(capacity, *written) unsigned char* buffer,
        unsigned capacity, _Out_ unsigned* written) = 0;
};
