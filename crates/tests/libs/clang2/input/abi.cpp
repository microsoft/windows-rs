#include "abi.h"
#include <stddef.h>

unsigned long AbiLayout(unsigned long index) {
    switch (index) {
    case 0: return sizeof(AbiPacket);
    case 1: return alignof(AbiPacket);
    case 2: return offsetof(AbiPacket, tag);
    case 3: return offsetof(AbiPacket, count);
    case 4: return offsetof(AbiPacket, scale);
    case 5: return offsetof(AbiPacket, value);
    default: return 0xffffffff;
    }
}

AbiPacket AbiRoundtrip(AbiPacket packet) {
    packet.tag += 1;
    packet.count += 7;
    packet.scale *= 2;
    packet.value -= 9;
    return packet;
}

class Native final : public IAbiDerived {
    long bias = 11;
    long __stdcall Base(long value) override { return value + bias; }
    double __stdcall Measure(double value) override { return value * 2; }
    unsigned long __stdcall Mutate(AbiPacket* packet, int* values, unsigned long count) override {
        *packet = AbiRoundtrip(*packet);
        for (unsigned long i = 0; i < count; ++i) {
            values[i] += 3;
        }
        return count;
    }
};

IAbiDerived* AbiGet() {
    static Native object;
    return &object;
}

long __stdcall AbiCall(
    IAbiDerived* object, long value, double measure,
    AbiPacket* packet, int* values, unsigned long count) {
    return object->Base(value) + static_cast<long>(object->Measure(measure))
        + static_cast<long>(object->Mutate(packet, values, count));
}
