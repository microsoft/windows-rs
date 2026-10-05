//! namespace EnumAliases
//! input first.h
enum INNER : int { First = 1 };
typedef INNER OUTER;
//! input second.h
enum INNER : unsigned int { Second = 2 };
typedef INNER OUTER;
