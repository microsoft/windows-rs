enum Signed : signed char { Negative = -128, Positive = 127 };
enum Wide : unsigned long long { High = 0xffffffffffffffffULL };
enum class Scoped : unsigned short { First = 0, Last = 65535 };
typedef enum State { Ready = -1, Done = 2, AlsoDone = 2 } State;
typedef State StateAlias;

struct EnumPacket {
    Signed small;
    Scoped scoped;
    State state;
    Wide wide;
};

struct __declspec(uuid("12345678-1234-1234-1234-123456789abc")) IEnums {
    virtual State __stdcall Get(Scoped value, StateAlias* output) = 0;
};

extern "C" StateAlias ConvertEnum(EnumPacket* packet, Wide wide);
