#include "layouts.h"
#include "packed.h"
#include <stddef.h>

unsigned long LayoutEvidence(unsigned long index) {
    switch (index) {
    case 0: return sizeof(LayoutChoice);
    case 1: return alignof(LayoutChoice);
    case 2: return offsetof(LayoutChoice, number);
    case 3: return offsetof(LayoutChoice, real);
    case 4: return offsetof(LayoutChoice, bytes);
    case 5: return sizeof(LayoutAligned);
    case 6: return alignof(LayoutAligned);
    case 7: return offsetof(LayoutAligned, bytes);
    case 8: return offsetof(LayoutAligned, number);
    case 9: return sizeof(LayoutPacket);
    case 10: return alignof(LayoutPacket);
    case 11: return offsetof(LayoutPacket, tag);
    case 12: return offsetof(LayoutPacket, number);
    case 13: return offsetof(LayoutPacket, low);
    case 14: return offsetof(LayoutPacket, high);
    case 15: return offsetof(LayoutPacket, real);
    case 16: return offsetof(LayoutPacket, Anonymous1);
    case 17: return offsetof(LayoutPacket, choices);
    case 18: return offsetof(LayoutPacket, named);
    case 19: return offsetof(LayoutPacket, named.value);
    case 20: return offsetof(LayoutPacket, named.marker);
    case 21: return offsetof(LayoutPacket, aligned);
    default: return 0xffffffff;
    }
}

void LayoutMutate(LayoutPacket* packet) {
    packet->tag += 1;
    packet->real *= 2;
    packet->Anonymous1 += 3;
    packet->choices[0].number += 5;
    packet->choices[1].real += 0.5;
    packet->named.value -= 7;
    packet->named.marker += 9;
    packet->aligned.number += 11;
}

void LayoutInvoke(LayoutCallback callback, LayoutPacket* packet) {
    callback(packet);
}

unsigned long PackedLayoutEvidence(unsigned long index) {
    switch (index) {
    case 0: return sizeof(Packed1);
    case 1: return alignof(Packed1);
    case 2: return offsetof(Packed1, tag);
    case 3: return offsetof(Packed1, value);
    case 4: return offsetof(Packed1, wide);
    case 5: return offsetof(Packed1, pointer);
    case 6: return sizeof(PackedChoice);
    case 7: return alignof(PackedChoice);
    case 8: return offsetof(PackedChoice, record);
    case 9: return offsetof(PackedChoice, bits);
    case 10: return sizeof(PackedAnonymous);
    case 11: return alignof(PackedAnonymous);
    case 12: return offsetof(PackedAnonymous, tag);
    case 13: return offsetof(PackedAnonymous, value);
    case 14: return offsetof(PackedAnonymous, halves);
    case 15: return sizeof(Packed2);
    case 16: return alignof(Packed2);
    case 17: return offsetof(Packed2, tag);
    case 18: return offsetof(Packed2, values);
    case 19: return offsetof(Packed2, choice);
    case 20: return sizeof(Packed4);
    case 21: return alignof(Packed4);
    case 22: return offsetof(Packed4, tag);
    case 23: return offsetof(Packed4, wide);
    case 24: return offsetof(Packed4, pointer);
    case 25: return sizeof(PackedContainer);
    case 26: return alignof(PackedContainer);
    case 27: return offsetof(PackedContainer, records);
    case 28: return offsetof(PackedContainer, tail);
    case 29: return sizeof(NaturalChoice);
    case 30: return alignof(NaturalChoice);
    case 31: return offsetof(NaturalChoice, value);
    case 32: return offsetof(NaturalChoice, real);
    case 33: return sizeof(PackedField);
    case 34: return alignof(PackedField);
    case 35: return offsetof(PackedField, tag);
    case 36: return offsetof(PackedField, choice);
    case 37: return sizeof(PackedGap);
    case 38: return alignof(PackedGap);
    case 39: return offsetof(PackedGap, __padding1);
    case 40: return offsetof(PackedGap, marker);
    case 41: return offsetof(PackedGap, value);
    default: return 0xffffffff;
    }
}

void PackedMutate(Packed1* packet) {
    packet->tag += 1;
    packet->value += 3;
    packet->wide += 5;
    packet->pointer = packet;
}

void PackedInvoke(PackedCallback callback, Packed1* packet) {
    callback(packet);
}
