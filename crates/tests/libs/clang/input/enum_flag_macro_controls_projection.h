//! namespace Flags
//! no-library

#define DEFINE_ENUM_FLAG_OPERATORS(type)
             typedef enum FLAGS { FLAGS_NONE = 0, FLAGS_ALL = -1 } FLAGS;
             DEFINE_ENUM_FLAG_OPERATORS(FLAGS)
