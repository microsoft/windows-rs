enum class Mode : signed char { Off = -1, On = 2 };
typedef Mode ModeAlias;
const ModeAlias SELECTED = Mode::Off;
enum Wide : unsigned long long { Maximum = 0xffffffffffffffffULL };
const Wide LARGE = Maximum;
const Wide NATIVE_WIDTH = static_cast<Wide>(sizeof(void*));
#define SELECTED_MACRO Mode::On
#define UNNAMED_VALUE static_cast<Mode>(3)
enum : short { ANONYMOUS_MEMBER = -2 };
#define ANONYMOUS_MACRO ANONYMOUS_MEMBER
enum __MIDL_Fixture_0100 : unsigned short { MIDL_MEMBER = 65535 };
#define MIDL_MACRO MIDL_MEMBER
