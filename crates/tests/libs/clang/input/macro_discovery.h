typedef unsigned short Code;
#define NUMBER 42
#define SHIFT (1ULL << 40)
#define TYPED ((Code) (L'\x01' | 0x80))
#define TEXT "backup"
#define WIDE L"restore"
#define WRAP(value) ((value) + 1)
#define INDIRECT WRAP(NUMBER)
#if defined(_M_IX86)
#define PLATFORM 32
#else
#define PLATFORM 64
#endif
struct Safe { int value; };
