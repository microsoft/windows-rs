#include "alignment_types.h"
#pragma pack(push, 1)
struct __declspec(align(16)) PackedAligned { char tag; int value; };
struct PackedContainer { Standard value; };
struct PackedArray { Container values[2]; };
#pragma pack(pop)
struct FieldAligned { char tag; __declspec(align(16)) int value; };
typedef int AlignedAlias __attribute__((aligned(16)));
struct AliasUser { AlignedAlias value; };
typedef const int ConstAlignedAlias __attribute__((aligned(16)));
struct ConstAliasUser { ConstAlignedAlias* value; };
struct Plain { int value; };
typedef Plain AlignedRecordAlias __attribute__((aligned(16)));
struct RecordAliasUser { AlignedRecordAlias* value; };
enum __attribute__((aligned(4))) AlignedEnum : int { One };
struct EnumUser { AlignedEnum value; };
struct Dependent { PackedAligned* value; };
struct Survivor { int value; };
