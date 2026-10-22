#ifndef CASE
#define CASE 0
#endif
struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IUnknown {
    virtual long __stdcall QueryInterface(const void* iid, void** value) = 0;
    virtual unsigned long __stdcall AddRef() = 0;
    virtual unsigned long __stdcall Release() = 0;
};
struct __declspec(uuid("af86e2e0-b12d-4c6a-9c5a-d7aa65101e90")) IInspectable : IUnknown {
    virtual long __stdcall GetIids(unsigned long* count, void** values) = 0;
    virtual long __stdcall GetRuntimeClassName(void** value) = 0;
    virtual long __stdcall GetTrustLevel(int* value) = 0;
};
namespace ABI::External {
#if CASE == 7
    struct __declspec(uuid("00000002-0000-0000-c000-000000000046")) IHandler : IUnknown {
#elif CASE == 10
    struct __declspec(uuid("3d52ad5b-147c-4387-8f04-f4eb9a71b483")) IHandler : IInspectable {
#else
    struct __declspec(uuid("3d52ad5b-147c-4387-8f04-f4eb9a71b483")) IHandler : IUnknown {
#endif
#if CASE == 8
        virtual long __stdcall Invoke(unsigned int value, unsigned long long* result) = 0;
#else
        virtual long __stdcall Invoke(unsigned int value, unsigned int* result) = 0;
#endif
#if CASE == 9
        virtual long __stdcall Extra() = 0;
#endif
    };
#if CASE == 1
    typedef struct __declspec(uuid("00000001-0000-0000-c000-000000000046")) IBuffer : IInspectable {
#else
    typedef struct __declspec(uuid("905a0fef-bc53-11df-8c49-001e4fc686da")) IBuffer : IInspectable {
#endif
#if CASE == 2
        virtual long __stdcall get_Capacity(unsigned long long* value) = 0;
#elif CASE == 3
        virtual long __stdcall get_Capacity(const unsigned int* value) = 0;
#elif CASE == 4
        virtual long __stdcall Capacity(unsigned int* value) = 0;
#elif CASE == 5
        virtual long __stdcall get_Capacity(unsigned int value) = 0;
#elif CASE == 6
        virtual unsigned long __stdcall get_Capacity(unsigned int* value) = 0;
#else
        virtual long __stdcall get_Capacity(unsigned int* value) = 0;
#endif
        virtual long __stdcall get_Length(unsigned int* value) = 0;
        virtual long __stdcall put_Length(unsigned int value) = 0;
    } BufferType;
    typedef IBuffer IBuffer;
}
namespace External {
    struct LocalHelper : IUnknown {
        virtual long __stdcall Access(unsigned char** value) = 0;
    };
}
struct ReferenceConsumer {
    ABI::External::IBuffer* buffer;
    External::LocalHelper* helper;
    ABI::External::IHandler* handler;
};
