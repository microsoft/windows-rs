typedef void* Handle;
Handle const ALIAS = (Handle)-1;
#define INVALID ((Handle)-1)
void* const NOTHING = nullptr;
const unsigned char* const READ_ONLY = (const unsigned char*)2;
void* const TRUNCATED32 = (void*)0xffffffffULL;
void* const HIGH_BIT = (void*)0x8000000000000000ULL;
