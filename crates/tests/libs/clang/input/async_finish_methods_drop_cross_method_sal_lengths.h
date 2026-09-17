//! namespace AsyncInterface
//! no-library
//! args -x c++ -fms-extensions

#define WRITES(c) __attribute__((annotate("_Out_writes_(" #c ")")))
             struct IUnknown { virtual int Query() = 0; };
             struct __declspec(uuid("12345678-1234-abcd-9876-0123456789ab"))              AsyncITest : IUnknown {
                 virtual int Begin_Read(unsigned count) = 0;
                 virtual int Finish_Read(WRITES(count) int* values) = 0;
             };
