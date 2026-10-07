#define UUID(id) __declspec(uuid(id))
#define ATTRIBUTES(message, id) __declspec(deprecated(message), uuid(id))
#define INTERFACE(id) struct UUID(id)
class __declspec(uuid("12345678-1234-1234-1234-123456789abc")) Direct;
class __declspec(deprecated("12345678-1234-1234-1234-123456789abc")) Deprecated;
class UUID("12345678-1234-1234-1234-123456789abc") Indirect;
class ATTRIBUTES("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa", "12345678-1234-1234-1234-123456789abc") Combined;
INTERFACE("12345678-1234-1234-1234-123456789abc") Nested;
#define UUID_KIND uuid
class __declspec(UUID_KIND("12345678-1234-1234-1234-123456789abc")) TokenMacro;
class __declspec(deprecated("__declspec(uuid(\"12345678-1234-1234-1234-123456789abc\"))")) Quoted;
