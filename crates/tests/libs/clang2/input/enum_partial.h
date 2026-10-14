enum SafeEnum { SafeValue = 7 };
struct Safe { SafeEnum value; };
enum Wide : unsigned long long { WideValue = 18446744073709551615ULL };
struct SecondSafe { Wide value; };
#if defined(_M_IX86)
enum Changing : bool { Changed = true };
#else
struct Changing { int value; };
#endif
struct Dependent { Changing* value; };
enum BadMember {
    Bad [[deprecated]] = 1
};
struct BadDependent { BadMember value; };
typedef BadMember* BadPointer;
typedef unsigned int Backing __attribute__((deprecated));
enum BackingEnum : Backing { BackingValue = 0 };
struct BackingUser { BackingEnum value; };
