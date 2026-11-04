#define FLAGS(name, storage, ...) \
    typedef __attribute__((annotate("Win32Metadata.FlagsEnum(" #__VA_ARGS__ ")", __VA_ARGS__))) storage name
#define OPTION_A 1
#define OPTION_B 2
const int OPTION_C = 4;

typedef unsigned long OPTIONS;
#if !defined(SECOND_ONLY)
FLAGS(OPTIONS, unsigned long, OPTION_A, OPTION_B);
#endif
#if !defined(FIRST_ONLY)
FLAGS(OPTIONS, unsigned long, OPTION_B, OPTION_C);
#endif
