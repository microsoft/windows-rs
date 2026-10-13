#include <sal.h>

#define INPUT LEAF
#define LEAF _In_
extern "C" void Before(INPUT int* value);
#undef LEAF
#define LEAF _Out_
extern "C" void After(INPUT int* value);

#define BUFFER _Out_writes_bytes_to_
extern "C" void Fill(BUFFER(capacity, *written) void* buffer,
                     _In_ unsigned capacity, _Out_ unsigned* written);

#define NOT_A_CONTRACT int
extern "C" void Plain(NOT_A_CONTRACT value);
