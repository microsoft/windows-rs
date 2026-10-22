typedef class ClassTag ClassTag;
class __declspec(uuid("12345678-9abc-def0-0102-030405060708")) ClassTag;
struct __declspec(uuid("ffffffff-abcd-1234-8001-0203040506ff")) StructTag;
#ifdef _M_IX86
class __declspec(uuid("00000004-0000-0000-0000-000000000000")) ArchTag;
#else
class __declspec(uuid("00000008-0000-0000-0000-000000000000")) ArchTag;
#endif
