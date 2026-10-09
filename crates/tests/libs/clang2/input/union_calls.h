union AbiByte { unsigned char bits; signed char signed_bits; };
union AbiHalf { unsigned short bits; short signed_bits; };
union AbiWord { unsigned long bits; float real; };
union AbiWide { unsigned long long bits; double real; };
union AbiLarge { unsigned long long bits[2]; double reals[2]; };
union AbiHuge { unsigned long long bits[3]; double reals[3]; };

#if defined(_MSC_VER) && !defined(__clang__)
#pragma warning(push)
#pragma warning(disable: 4201)
#endif
struct AbiAnonymous {
    union { unsigned long long bits; double real; };
    struct { unsigned long value; unsigned long tag; } tail;
};
#if defined(_MSC_VER) && !defined(__clang__)
#pragma warning(pop)
#endif

typedef double (*UnionCallback)(
    unsigned long head, double first, AbiByte a, AbiHalf b, AbiWord c,
    AbiWide d, AbiLarge e, AbiHuge f, AbiAnonymous g, double last, unsigned long tail);

struct __declspec(uuid("b5b15b61-4ef2-40ab-9723-61b6b4e8e927")) IUnionAbi {
    virtual double __stdcall Measure(
        unsigned long head, double first, AbiByte a, AbiHalf b, AbiWord c,
        AbiWide d, AbiLarge e, AbiHuge f, AbiAnonymous g, double last, unsigned long tail) = 0;
};

extern "C" double UnionMeasure(
    unsigned long head, double first, AbiByte a, AbiHalf b, AbiWord c,
    AbiWide d, AbiLarge e, AbiHuge f, AbiAnonymous g, double last, unsigned long tail);
extern "C" double UnionInvoke(
    UnionCallback callback, unsigned long head, double first, AbiByte a, AbiHalf b, AbiWord c,
    AbiWide d, AbiLarge e, AbiHuge f, AbiAnonymous g, double last, unsigned long tail);
extern "C" IUnionAbi* UnionObject();
extern "C" double UnionVirtualInvoke(
    IUnionAbi* object, unsigned long head, double first, AbiByte a, AbiHalf b, AbiWord c,
    AbiWide d, AbiLarge e, AbiHuge f, AbiAnonymous g, double last, unsigned long tail);
