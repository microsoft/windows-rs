enum class Mode : signed char { Off = -1, On = 2 };
typedef Mode ModeAlias;
const ModeAlias SELECTED = Mode::Off;
enum Wide : unsigned long long { Maximum = 0xffffffffffffffffULL };
const Wide LARGE = Maximum;
const Wide NATIVE_WIDTH = static_cast<Wide>(sizeof(void*));
#define SELECTED_MACRO Mode::On
#define UNNAMED_VALUE static_cast<Mode>(3)
