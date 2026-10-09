#include "union_calls.h"

double UnionMeasure(
    unsigned long head, double first, AbiByte a, AbiHalf b, AbiWord c,
    AbiWide d, AbiLarge e, AbiHuge f, AbiAnonymous g, double last, unsigned long tail) {
    return head + first + a.bits * 2 + b.bits * 3 + c.real * 5 + d.real * 7
        + e.bits[0] * 11 + e.bits[1] * 13
        + f.bits[0] * 17 + f.bits[1] * 19 + f.bits[2] * 23
        + g.real * 29 + g.tail.value * 31 + g.tail.tag * 37 + last * 41 + tail * 43;
}

double UnionInvoke(
    UnionCallback callback, unsigned long head, double first, AbiByte a, AbiHalf b, AbiWord c,
    AbiWide d, AbiLarge e, AbiHuge f, AbiAnonymous g, double last, unsigned long tail) {
    return callback(head, first, a, b, c, d, e, f, g, last, tail);
}

class NativeUnion final : public IUnionAbi {
    double __stdcall Measure(
        unsigned long head, double first, AbiByte a, AbiHalf b, AbiWord c,
        AbiWide d, AbiLarge e, AbiHuge f, AbiAnonymous g, double last, unsigned long tail) override {
        return UnionMeasure(head, first, a, b, c, d, e, f, g, last, tail);
    }
};

IUnionAbi* UnionObject() {
    static NativeUnion object;
    return &object;
}

double UnionVirtualInvoke(
    IUnionAbi* object, unsigned long head, double first, AbiByte a, AbiHalf b, AbiWord c,
    AbiWide d, AbiLarge e, AbiHuge f, AbiAnonymous g, double last, unsigned long tail) {
    return object->Measure(head, first, a, b, c, d, e, f, g, last, tail);
}
