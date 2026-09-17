// An inline anonymous enum used directly as a field type has no name to
// reference. The field takes the enum's underlying integer type and the
// constants are still emitted as free constants, exactly as wsdxmldom.h's
// `WSDXML_NODE.Type` (`enum { ElementType, TextType } Type;`).
struct Node {
    enum {
        ElementType,
        TextType
    } Type;
    int Parent;
};
