// Bit-fields have no winmd representation, so consecutive bit-fields are
// coalesced into backing integer fields named `_bitfield` (when a type has a
// single backing field) or `_bitfield1`, `_bitfield2`, … (numbered in
// declaration order) when it has several.

// Three sub-byte fields that pack into a single byte -> one `_bitfield: u8`.
struct Flags8 {
    unsigned char a : 1;
    unsigned char b : 2;
    unsigned char c : 5;
};

// Two full bytes -> two backing fields `_bitfield1`, `_bitfield2`.
struct Flags16 {
    unsigned char low : 8;
    unsigned char high : 8;
};

// Signed bit-fields keep a signed backing type.
struct Signed {
    int x : 4;
    int y : 4;
};

// A regular member between two bit-field runs: backing-field numbering is
// global across the whole type, so the second run is `_bitfield2`.
struct Mixed {
    unsigned int first : 4;
    unsigned int tag;
    unsigned int second : 4;
};

// Differing storage-unit sizes force a new backing field even when bits remain.
struct Widths {
    unsigned char small : 4;
    unsigned long large : 4;
};

// A zero-width bit-field forces the next field onto a fresh storage unit.
struct ZeroWidth {
    unsigned int a : 4;
    unsigned int : 0;
    unsigned int b : 4;
};

// An anonymous padding bit-field consumes bits within the current unit but
// emits no member, so this remains a single `_bitfield: u32`.
struct Padding {
    unsigned int a : 4;
    unsigned int : 4;
    unsigned int b : 8;
};

// A union whose leading bit-fields coalesce into one backing field that
// overlaps a wider member (the RTL_BALANCED_NODE pattern).
union Overlap {
    unsigned char flags_a : 1;
    unsigned char flags_b : 2;
    unsigned long long whole;
};

// Packing applies to the coalesced layout as well.
#pragma pack(push, 1)
struct PackedBits {
    unsigned short a : 3;
    unsigned char tail;
};
#pragma pack(pop)
