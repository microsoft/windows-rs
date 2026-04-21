// Tests #define constant patterns not covered by const.h:
//   - negated integer literal  (- LITERAL arm)
//   - LL suffix                (Value::I64 arm)
//   - ULL suffix               (Value::U64 arm)
//   - single-paren typed cast  ((TYPE)VALUE arm)
//   - negated double-paren cast (((TYPE)-VALUE) arm)
//   - negated single-paren cast ((TYPE)-VALUE arm)

/* Negated integer literal: parse_body [(Punct,"-"),(Lit,...)] arm */
#define NEG_ONE -1
#define NEG_LARGE -100

/* LL suffix → Value::I64 */
#define BIG_LL 0x100000000LL

/* ULL suffix → Value::U64 */
#define BIG_ULL 0x100000000ULL

/* Typedef used as cast target for the typed-cast tests below */
typedef long long TestStatusCode;

/* Single-paren typed cast: (TYPE)VALUE */
#define STATUS_ZERO (TestStatusCode)0

/* Negated double-paren typed cast: ((TYPE)-VALUE) */
#define STATUS_NEG ((TestStatusCode)-1)

/* Negated single-paren typed cast: (TYPE)-VALUE */
#define STATUS_NEG2 (TestStatusCode)-2
