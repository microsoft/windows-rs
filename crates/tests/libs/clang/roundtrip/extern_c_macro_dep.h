// Defines EXTERN_C in a separate included header so that the macro's spelling
// location is not in the main file, exercising the expansion-location fallback
// in the clang parser.
#define EXTERN_C extern "C"
