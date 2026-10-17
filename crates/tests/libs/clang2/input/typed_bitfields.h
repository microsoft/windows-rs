typedef signed int NativeInt;
typedef NativeInt OtherInt;
enum class Mode : signed int { Negative = -1, Positive = 1 };
typedef Mode ModeAlias;
typedef signed long long WideInt;
enum State : unsigned int { Off = 0, On = 1 };

struct MixedBits {
    unsigned int positive : 3;
    signed int negative : 5;
    unsigned int flag : 1;
    signed int sign : 1;
    Mode mode : 3;
    State state : 2;
    OtherInt alias : 5;
    ModeAlias alternate : 3;
};

struct ReverseBits {
    signed int negative : 5;
    unsigned int positive : 3;
};

struct BoolBits {
    bool enabled : 1;
    bool wide : 3;
    unsigned char other : 4;
};

struct FullSignedBits {
    WideInt value : 64;
};

union TypedUnion {
    Mode mode : 3;
    signed int sign : 1;
    unsigned int storage;
};

struct DependentBits {
    MixedBits mixed;
    BoolBits flags;
};
