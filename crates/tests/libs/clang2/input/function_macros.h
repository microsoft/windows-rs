extern "C" int __stdcall Native(unsigned count, const unsigned* values);
extern "C" int __stdcall Other(unsigned count, const unsigned* values);
extern "C" int Unsupported(int value) noexcept(false);
extern "C" int Self(int value);
namespace Scoped { extern "C" int Hidden(int value); }
extern "C" int Shadow(unsigned value);
extern "C" int Narrow(const char* value);

#define Public Native
#define Nested Public
#define Parenthesized (Native)
#define Comma (0, Native)
#define Pointer (&Native)
#define UnsupportedAlias Unsupported
#define Self Self
#define ScopedAlias Scoped::Hidden
#define Shadow Narrow
