struct Anchor { int value; };
struct Incomplete;
template <typename T> struct NeedsComplete { T value; };

#define A_GOOD 11
#define B_CLOSE ) ]
#define C_GOOD 22
#define D_OPEN (
#define E_GOOD 33
#define F_BRACE {
#define G_GOOD 44
#define H_NAMESPACE namespace Broken {
#define I_GOOD 55
#define J_LAMBDA []() {
#define K_GOOD 66
#define L_SEMICOLON 1; }
#define M_GOOD 77
#define N_TEMPLATE sizeof(NeedsComplete<Incomplete>)
#define O_GOOD 88
#define P_EMPTY
#define Q_GOOD 99
#define R_TYPE unsigned int
#define S_GOOD 111
#define T_FIELDS 1, 2, 3
#define U_GOOD (1, 2, 3)
#define V_BAD00 Missing00
#define V_BAD01 Missing01
#define V_BAD02 Missing02
#define V_BAD03 Missing03
#define V_BAD04 Missing04
#define V_BAD05 Missing05
#define V_BAD06 Missing06
#define V_BAD07 Missing07
#define V_BAD08 Missing08
#define V_BAD09 Missing09
#define V_BAD10 Missing10
#define V_BAD11 Missing11
#define V_BAD12 Missing12
#define V_BAD13 Missing13
#define V_BAD14 Missing14
#define V_BAD15 Missing15
#define V_BAD16 Missing16
#define V_BAD17 Missing17
#define V_BAD18 Missing18
#define V_BAD19 Missing19
#define V_BAD20 Missing20
#define V_BAD21 Missing21
#define V_BAD22 Missing22
#define V_BAD23 Missing23
#define W_GOOD 222
#define X_OPEN 1); struct {
#define Y_GOOD 333
#define Z_CLOSE 1; }
#define Z_GOOD 444
