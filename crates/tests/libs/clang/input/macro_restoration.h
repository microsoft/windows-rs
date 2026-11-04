typedef unsigned int Word;

#define RESTORED ((Word)32)
#pragma push_macro("RESTORED")
#undef RESTORED
#define RESTORED ((unsigned short)99)
#pragma push_macro("RESTORED")
#undef RESTORED
#define RESTORED 123LL
#pragma pop_macro("RESTORED")
const unsigned short Inner = RESTORED;
#pragma pop_macro("RESTORED")

#define SHADOWED 32
#pragma push_macro("SHADOWED")
#undef SHADOWED
const unsigned short SHADOWED = 7;
#pragma pop_macro("SHADOWED")
