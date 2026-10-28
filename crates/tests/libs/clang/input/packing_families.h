#ifdef PACK_TWO
#pragma pack(push, 2)
struct SecondPacking { char value; };
typedef SecondPacking PublicPacking;
#else
#pragma pack(push, 1)
struct FirstPacking { char value; };
typedef FirstPacking PublicPacking;
#endif
#pragma pack(pop)
