#include <sal.h>
#include <specstrings.h>

#define API_RESULT int

extern "C" {
    API_RESULT Read(
        /* [annotation] */ _Out_writes_bytes_to_(capacity, *written) void* buffer,
        /* [annotation][in] */ _In_ unsigned int capacity,
        _Out_opt_ unsigned int* written);
    API_RESULT Write(
        _In_reads_bytes_(capacity) const void* buffer,
        _In_ unsigned int capacity,
        _Out_opt_ unsigned int* written);
    API_RESULT Query(
        /* [in] */ _In_ const int* identity,
        /* [iid_is][out] */ _COM_Outptr_ void** object);
    API_RESULT Unnamed(_In_ unsigned int, _In_opt_ const int* value);
    API_RESULT Unknown(void* buffer);
}
