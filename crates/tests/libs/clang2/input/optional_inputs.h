#ifndef _In_opt_
#ifdef __clang__
#define _In_opt_ __attribute__((annotate("_In_opt_")))
#else
#define _In_opt_
#endif
#endif

#ifdef _WIN64
typedef unsigned long long OptionalWord;
typedef long long OptionalSigned;
#else
typedef unsigned long OptionalWord;
typedef long OptionalSigned;
#endif
enum OptionalKind : unsigned long { OptionalNone = 0, OptionalReady = 7 };
struct OptionalPair { long value; };
typedef long long (__stdcall* OptionalCallback)(_In_opt_ long long value);

extern "C" long long OptionalInteger(_In_opt_ long long value);
extern "C" OptionalWord OptionalUnsigned(_In_opt_ OptionalWord value);
extern "C" OptionalSigned OptionalSignedWord(_In_opt_ OptionalSigned value);
extern "C" OptionalKind OptionalEnum(_In_opt_ OptionalKind value);
extern "C" float OptionalFloat(_In_opt_ float value);
extern "C" long OptionalRecord(_In_opt_ OptionalPair value);
extern "C" long long OptionalPointer(_In_opt_ const long long* value);
extern "C" long long OptionalInvoke(_In_opt_ OptionalCallback callback, _In_opt_ long long value);
