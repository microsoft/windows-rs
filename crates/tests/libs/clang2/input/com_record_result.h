struct Small { int value; };
struct Large { double first; double second; double third; };
using Alias = Large;

struct __declspec(uuid("00000001-0000-0000-c000-000000000046")) IRecordResult {
    virtual RESULT __stdcall Get() = 0;
};
