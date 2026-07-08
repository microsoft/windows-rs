// A flexible array member (a trailing unsized array) is captured faithfully as
// a zero-length fixed array rather than a guessed placeholder length.
struct Packet {
    unsigned int length;
    unsigned char data[];
};
