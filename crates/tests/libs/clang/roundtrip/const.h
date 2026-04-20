/* Simple integer constant (hex, no suffix). */
#define FACILITY_DEBUGGER 0x1

/* Integer constant with L suffix (LONG = i32 on Windows). */
#define E_NOINTERFACE 0x80004002L

/* Unsigned integer constant. */
#define FLAG_ALL 0xFFFFFFFFU

/* String constant. */
#define UDAT_YEAR_NUM_MONTH_DAY "yMd"

/* Typed cast constant — double-paren Win32 style. */
typedef long long TestStatus;
#define STATUS_CLIP_LICENSE_NOT_FOUND ((TestStatus)0xC0EA0002L)

/* Complex: arithmetic on another macro (evaluator fallback). */
#define FACILITY_DEBUGGER_PLUS_TEN FACILITY_DEBUGGER + 10

/* Complex: left-shift of another macro (evaluator fallback). */
#define FACILITY_DEBUGGER_SHIFTED (FACILITY_DEBUGGER << 16)
