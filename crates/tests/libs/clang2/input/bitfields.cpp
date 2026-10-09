#include "bitfields.h"
#include <stddef.h>

unsigned long BitLayoutEvidence(unsigned long index) {
    switch (index) {
    case 0: return sizeof(BitUnits);
    case 1: return alignof(BitUnits);
    case 2: return offsetof(BitUnits, __bitfield0);
    case 3: return offsetof(BitUnits, end);
    case 4: return sizeof(BitPacked);
    case 5: return alignof(BitPacked);
    case 6: return offsetof(BitPacked, tag);
    case 7: return offsetof(BitPacked, value);
    case 8: return sizeof(BitNested);
    case 9: return alignof(BitNested);
    case 10: return offsetof(BitNested, entries);
    case 11: return offsetof(BitNested, choice);
    case 12: return sizeof(BitFull);
    case 13: return alignof(BitFull);
    default: return 0xffffffff;
    }
}

unsigned long long BitRead(const BitUnits* value, unsigned long index) {
    switch (index) {
    case 0: return value->flag;
    case 1: return value->mode;
    case 2: return value->low;
    case 3: return value->high;
    case 4: return value->first;
    case 5: return value->second;
    case 6: return value->next;
    case 7: return value->after;
    case 8: return value->wide;
    case 9: return value->tail;
    default: return 0xffffffffffffffff;
    }
}

void BitWrite(BitUnits* value, unsigned long index, unsigned long long input) {
    switch (index) {
    case 0: value->flag = static_cast<unsigned char>(input); break;
    case 1: value->mode = static_cast<unsigned char>(input); break;
    case 2: value->low = static_cast<unsigned short>(input); break;
    case 3: value->high = static_cast<unsigned short>(input); break;
    case 4: value->first = static_cast<unsigned long>(input); break;
    case 5: value->second = static_cast<unsigned long>(input); break;
    case 6: value->next = static_cast<unsigned long>(input); break;
    case 7: value->after = static_cast<unsigned long>(input); break;
    case 8: value->wide = input; break;
    case 9: value->tail = input; break;
    }
}

unsigned long BitPackedRead(const BitPacked* value) {
    return value->enabled | (value->level << 1);
}

void BitPackedWrite(BitPacked* value, unsigned long input) {
    value->enabled = input;
    value->level = input >> 1;
}

void BitInvoke(BitCallback callback, BitUnits* value, BitPacked* packed) {
    callback(value, packed);
}
