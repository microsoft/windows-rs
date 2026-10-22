#define CHECK __attribute__((annotate("windows_clang:sal:2:_Check_return_\n()\n()")))
#define INSPECT __attribute__((annotate("windows_clang:sal:2:_Must_inspect_result_\n()\n()")))
#define SUCCESS __attribute__((annotate("windows_clang:sal:2:_Success_\n(return >= 0)\n(return >= 0)")))
#define TYPE_SUCCESS __attribute__((annotate("windows_clang:sal:2:_Return_type_success_\n(return >= 0)\n(return >= 0)")))
#define OUT __attribute__((annotate("windows_clang:sal:2:_Out_\n()\n()")))
#define POST __attribute__((annotate("windows_clang:sal:2:_Post_\n()\n()")))
#define CONDITIONAL __attribute__((annotate("windows_clang:sal:2:_When_\n(value > 0, _Check_return_)\n(value > 0, _Check_return_)")))
#define OLD __attribute__((annotate("windows_clang:sal:1:_Check_return_\n()\n()")))
#define ARGUMENTS __attribute__((annotate("windows_clang:sal:2:_Check_return_\n(unexpected)\n(unexpected)")))
#define EXPANDED __attribute__((annotate("windows_clang:sal:2:_Must_inspect_result_\n()\n(unexpected)")))

typedef TYPE_SUCCESS int Status;
typedef INSPECT int InspectedValue;
INSPECT const int GlobalValue = 7;
struct InspectedField { INSPECT int value; };

extern "C" {
    CHECK int Checked();
    INSPECT int Inspected();
    CHECK INSPECT CHECK int Combined();
    SUCCESS CHECK int Successful();
    CHECK Status ReadStatus();
    void Outputs(OUT INSPECT int* value);
    POST CHECK int Scoped();
    CONDITIONAL int Conditional(int value);
    OLD int OldVersion();
    ARGUMENTS int UnexpectedArguments();
    EXPANDED int ExpandedArguments();
}

typedef CHECK int (__stdcall *CheckedCallback)(int value);
typedef INSPECT int (__stdcall *InspectedCallback)(int value);
typedef CheckedCallback CheckedAlias;
typedef INSPECT CheckedCallback CombinedAlias;
typedef SUCCESS CHECK int (__stdcall *SuccessCallback)(int value);
typedef SuccessCallback SuccessAlias;

struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IResults {
    virtual CHECK int __stdcall Checked() = 0;
    virtual INSPECT int __stdcall Inspected() = 0;
    virtual CHECK INSPECT int __stdcall Combined() = 0;
    virtual void __stdcall Outputs(OUT INSPECT int* value) = 0;
};
