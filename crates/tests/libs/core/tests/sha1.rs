//! SHA-1 known-answer tests for the internal digest used to derive WinRT GUIDs.

use windows_core::imp::{ConstBuffer, sha1};

fn hash(input: &[u8]) -> String {
    sha1(&ConstBuffer::from_slice(input)).to_string()
}

#[test]
fn known_answers() {
    // RFC 3174 / NIST test vectors. WinRT delegate GUIDs derive from these hashes, so a
    // regression here silently corrupts IIDs and breaks `QueryInterface` at runtime.
    assert_eq!(hash(b""), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    assert_eq!(hash(b"abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");

    // 56 bytes: forces a second padded block (length field spills past the first block).
    assert_eq!(
        hash(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        "84983e441c3bd26ebaae4aa1f95129e5e54670f1"
    );

    // Longer than one 64-byte block.
    assert_eq!(
        hash(b"The quick brown fox jumps over the lazy dog"),
        "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12"
    );
}
