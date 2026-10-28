namespace ABI::External {
#if CASE == 1
    struct __declspec(uuid("00000001-0000-0000-c000-000000000046")) IBuffer;
#else
    struct IBuffer;
#endif
}
struct ForwardConsumer {
    ABI::External::IBuffer* buffer;
};
