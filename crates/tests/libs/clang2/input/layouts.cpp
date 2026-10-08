#include "layouts.h"
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
