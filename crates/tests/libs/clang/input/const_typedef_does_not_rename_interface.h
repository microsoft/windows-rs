//! namespace InterfaceAlias
//! no-library
//! args -x c++ -fms-extensions

struct __declspec(uuid("12345678-1234-abcd-9876-0123456789ab")) ITest {
                 virtual int Method() = 0;
             };
             typedef const ITest CTest;
