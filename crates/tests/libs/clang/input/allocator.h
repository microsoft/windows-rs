#ifndef ALLOCATOR
#define ALLOCATOR __declspec(allocator)
#endif

extern "C" ALLOCATOR void* __stdcall Allocate(unsigned int size);
extern "C" ALLOCATOR void* __stdcall Reallocate(void* previous, unsigned int size);

struct __declspec(uuid("12345678-1234-1234-1234-123456789abc")) Factory {
    virtual ALLOCATOR void* __stdcall Allocate(unsigned int size) = 0;
};
