#include <sal.h>

#if CASE == 1
typedef void (*Invalid)(_Outptr_result_buffer_(count) unsigned char* buffer, unsigned int count);
#elif CASE == 2
typedef void (*Invalid)(_Outptr_result_buffer_(count) unsigned char** buffer, float count);
#elif CASE == 3
typedef void (*Invalid)(_Outptr_result_buffer_(*count) unsigned char** buffer, unsigned int count);
#elif CASE == 4
typedef void (*Callback)();
typedef void (*Invalid)(_Outptr_result_buffer_(count) Callback* buffer, unsigned int count);
#elif CASE == 5
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IObject {
    virtual void __stdcall Invoke() = 0;
};
typedef void (*Invalid)(_Outptr_result_buffer_(count) IObject** buffer, unsigned int count);
#elif CASE == 6
typedef void (*Invalid)(_Outptr_result_buffer_(*count) unsigned char** buffer, float* count);
#endif
