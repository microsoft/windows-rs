//! namespace LegacyAnnotations
//! library api.dll

#define IN
             #define OUT
             #define OPTIONAL
             typedef void (*CALLBACK)(IN OUT void* context OPTIONAL);
             extern "C" void Update(IN const int* input, OUT int* output OPTIONAL);
