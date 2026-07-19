//! A tiny, dependency-free pseudo-random number generator (xorshift64), used to
//! place mines. A real game might use the `rand` crate, but the sample keeps its
//! dependency list to just `windows-composition` and `windows-window`.

/// A xorshift64 pseudo-random number generator.
pub struct Rng(u64);

impl Rng {
    /// Creates a generator from a seed. Any non-zero seed produces a full-period
    /// sequence; a zero seed is nudged to `1`.
    pub fn new(seed: u64) -> Self {
        Self(if seed == 0 { 1 } else { seed })
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// Returns a value uniformly distributed in `0..bound`. `bound` must be
    /// non-zero.
    pub fn next_range(&mut self, bound: usize) -> usize {
        (self.next_u64() % bound as u64) as usize
    }
}
