#define MIDL_INTERFACE(x) struct __declspec(uuid(x))

MIDL_INTERFACE("7c185885-a015-4cac-9411-0f4fb39b1f3a")
IFoo
{
public:
};

class __declspec(uuid("cafecafe-1234-5678-9abc-def012345678"))
IBar
{
public:
};
