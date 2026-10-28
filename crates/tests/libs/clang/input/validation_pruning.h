#ifndef FIELD_TYPE
#define FIELD_TYPE int
#endif
#ifndef RESULT_TYPE
#define RESULT_TYPE int
#endif
#ifdef COMPLETE_DEPENDENCY
struct Broken { int value; };
#else
struct Broken { int& value; };
#endif
#ifdef COMPLETE_ENUM
enum BrokenEnum { BrokenValue };
typedef int BrokenAlias;
#else
enum __attribute__((annotate("unsupported"))) BrokenEnum { BrokenValue };
typedef int BrokenAlias __attribute__((annotate("unsupported")));
#endif
struct EnumDependent { BrokenEnum value; };
struct AliasDependent { BrokenAlias value; };
struct Shared { FIELD_TYPE value; Broken* dependency; };
extern "C" RESULT_TYPE InspectShared(Shared* value);
struct Safe { int value; };
